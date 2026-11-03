param(
    [string]$RuntimeArchive,
    [string]$ModelFile,
    [string]$FrameworkPackage,
    [string]$Python = 'python'
)
$ErrorActionPreference = 'Stop'
$ProgressPreference = 'SilentlyContinue'
Set-StrictMode -Version Latest
$repo = Split-Path $PSScriptRoot -Parent
Push-Location $repo
try {
    $config = Get-Content packaging/store/store-config.json -Raw | ConvertFrom-Json
    $runtimeLock = Get-Content scripts/runtime-lock.json -Raw | ConvertFrom-Json
    $buildRoot = Join-Path $repo ('target/store-' + [guid]::NewGuid().ToString('N'))
    New-Item -ItemType Directory -Path $buildRoot -Force | Out-Null
    $diagnostics = Join-Path $repo 'target/store-diagnostics'
    New-Item -ItemType Directory -Path $diagnostics -Force | Out-Null
    $sdkRoot = Join-Path ${env:ProgramFiles(x86)} 'Windows Kits/10'
    $sdk = Get-ChildItem (Join-Path $sdkRoot 'bin') -Directory |
        Where-Object { $_.Name -match '^10\.' -and (Test-Path (Join-Path $_.FullName 'x64/MakeAppx.exe')) } |
        Sort-Object { [version]$_.Name } -Descending | Select-Object -First 1
    if (-not $sdk) { throw 'Windows SDK MakeAppx is required' }
    $makeAppx = Join-Path $sdk.FullName 'x64/MakeAppx.exe'
    if (-not $FrameworkPackage) {
        $frameworkRoot = Join-Path ${env:ProgramFiles(x86)} 'Microsoft SDKs/Windows Kits/10/ExtensionSDKs/Microsoft.VCLibs.Desktop'
        $candidates = @(Get-ChildItem $frameworkRoot -Recurse -Filter '*.appx' -File -ErrorAction SilentlyContinue |
            Where-Object { $_.FullName -match '[\\/]Appx[\\/]Retail[\\/]x64[\\/]' })
        [ordered]@{ windows_sdk = $sdk.Name; framework_sdk_present = (Test-Path $frameworkRoot)
            retail_x64_candidates = @($candidates | ForEach-Object Name) } |
            ConvertTo-Json | Set-Content (Join-Path $diagnostics 'tooling.json') -Encoding utf8
        if ($candidates.Count -eq 0) {
            throw 'Official x64 VCLibs Desktop framework package was not found. Supply -FrameworkPackage from the Microsoft Desktop framework SDK; do not copy System32 DLLs.'
        }
        $FrameworkPackage = ($candidates | Sort-Object FullName -Descending | Select-Object -First 1).FullName
    }
    $FrameworkPackage = (Resolve-Path -LiteralPath $FrameworkPackage).Path
    $signature = Get-AuthenticodeSignature -LiteralPath $FrameworkPackage
    if ($signature.Status -ne 'Valid' -or $signature.SignerCertificate.Subject -notlike '*O=Microsoft Corporation*') {
        throw 'The supplied framework must have a valid Microsoft signature'
    }
    $frameworkExtract = Join-Path $buildRoot 'framework'
    & $makeAppx unpack /p $FrameworkPackage /d $frameworkExtract
    if ($LASTEXITCODE -ne 0) { throw 'Official framework unpack failed' }
    [xml]$frameworkManifest = Get-Content (Join-Path $frameworkExtract 'AppxManifest.xml') -Raw
    $frameworkIdentity = $frameworkManifest.Package.Identity
    $microsoftPublisher = 'CN=Microsoft Corporation, O=Microsoft Corporation, L=Redmond, S=Washington, C=US'
    if ($frameworkIdentity.Name -ne 'Microsoft.VCLibs.140.00.UWPDesktop' -or
        $frameworkIdentity.Publisher -ne $microsoftPublisher -or $frameworkIdentity.ProcessorArchitecture -ne 'x64') {
        throw 'The framework identity is not the official x64 Desktop runtime'
    }
    $runtimeDll = @(Get-ChildItem $frameworkExtract -Filter msvcp140.dll -File -Recurse)
    if ($runtimeDll.Count -ne 1) { throw 'Expected one x64 Desktop framework DLL directory' }
    $frameworkDllDirectory = $runtimeDll[0].DirectoryName
    $frameworkReport = [ordered]@{
        name = [string]$frameworkIdentity.Name; publisher = [string]$frameworkIdentity.Publisher
        version = [string]$frameworkIdentity.Version; architecture = 'x64'; probe_passed = $false
        package_sha256 = (Get-FileHash $FrameworkPackage -Algorithm SHA256).Hash.ToLowerInvariant()
        dll_file_version = $runtimeDll[0].VersionInfo.FileVersion
    }
    $frameworkReport | ConvertTo-Json | Set-Content (Join-Path $diagnostics 'framework.json') -Encoding utf8
    if (-not $RuntimeArchive) {
        $RuntimeArchive = Join-Path $buildRoot 'runtime.zip'
        & curl.exe --fail --location --silent --show-error --retry 3 --proto '=https' --proto-redir '=https' --output $RuntimeArchive $runtimeLock.url
        if ($LASTEXITCODE -ne 0) { throw 'Pinned runtime download failed' }
    }
    if ((Get-FileHash $RuntimeArchive -Algorithm SHA256).Hash.ToLowerInvariant() -ne $runtimeLock.sha256) {
        throw 'Runtime archive checksum mismatch'
    }
    foreach ($test in @('tests/test_stage_bundle.py', 'tests/test_stage_store.py')) {
        & $Python $test
        if ($LASTEXITCODE -ne 0) { throw "Packaging tests failed: $test" }
    }
    & cargo test --locked
    if ($LASTEXITCODE -ne 0) { throw 'App tests failed' }
    & cargo build --release --locked --target x86_64-pc-windows-msvc
    if ($LASTEXITCODE -ne 0) { throw 'App build failed' }
    & $Python tests/test_greetings.py target/x86_64-pc-windows-msvc/release/lugha-desk.exe
    if ($LASTEXITCODE -ne 0) { throw 'Greeting integration failed' }
    # Use the existing verifier to extract only the allowlisted runtime in a fixture stage.
    $runtimeStage = Join-Path $buildRoot 'runtime-probe-payload'
    & $Python scripts/stage_bundle.py --app (Join-Path $repo 'target/x86_64-pc-windows-msvc/release/lugha-desk.exe') --runtime $RuntimeArchive --lock scripts/runtime-lock.json --notices packaging/notices --output $runtimeStage
    if ($LASTEXITCODE -ne 0) { throw 'Build the release app before Store runtime staging' }
    $vswhere = Join-Path ${env:ProgramFiles(x86)} 'Microsoft Visual Studio/Installer/vswhere.exe'
    $visualStudio = & $vswhere -latest -products '*' -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath
    if ($LASTEXITCODE -ne 0 -or -not $visualStudio) { throw 'Visual C++ compiler is required for the isolated dependency probe' }
    $tools = Get-ChildItem (Join-Path $visualStudio 'VC/Tools/MSVC') -Directory | Sort-Object { [version]$_.Name } -Descending | Select-Object -First 1
    $compiler = Join-Path $tools.FullName 'bin/Hostx64/x64/cl.exe'
    $probe = Join-Path $buildRoot 'framework-probe.exe'
    $previousPath = $env:PATH
    try {
        $env:PATH = (Split-Path $compiler -Parent) + ';' + $previousPath
        & $compiler /nologo /W4 /WX /MT /DUNICODE /D_UNICODE "/I$(Join-Path $tools.FullName 'include')" "/I$(Join-Path $sdkRoot ('Include/' + $sdk.Name + '/shared'))" "/I$(Join-Path $sdkRoot ('Include/' + $sdk.Name + '/um'))" "/I$(Join-Path $sdkRoot ('Include/' + $sdk.Name + '/ucrt'))" "/Fe:$probe" "/Fo:$(Join-Path $buildRoot 'framework-probe.obj')" tests/framework_probe.c /link "/LIBPATH:$(Join-Path $tools.FullName 'lib/x64')" "/LIBPATH:$(Join-Path $sdkRoot ('Lib/' + $sdk.Name + '/um/x64'))" "/LIBPATH:$(Join-Path $sdkRoot ('Lib/' + $sdk.Name + '/ucrt/x64'))" kernel32.lib
        if ($LASTEXITCODE -ne 0) { throw 'Native framework probe compile failed' }
    } finally { $env:PATH = $previousPath }
    & $Python tests/check_framework_probe.py --probe $probe --framework $frameworkDllDirectory --runtime (Join-Path $runtimeStage 'runtime')
    if ($LASTEXITCODE -ne 0) { throw 'Pinned runtime and Store framework are incompatible; no Store package will be published' }
    $frameworkReport.probe_passed = $true
    $frameworkJson = Join-Path $buildRoot 'framework.json'
    $frameworkReport | ConvertTo-Json | Set-Content $frameworkJson -Encoding utf8
    $frameworkReport | ConvertTo-Json | Set-Content (Join-Path $diagnostics 'framework.json') -Encoding utf8
    # Never download over an explicitly supplied model. A bad cached model also fails closed.
    if (-not $ModelFile) {
        $ModelFile = Join-Path $buildRoot $config.model.file
        & curl.exe --fail --location --silent --show-error --retry 3 --connect-timeout 30 --max-time 1800 --proto '=https' --proto-redir '=https' --output $ModelFile $config.model.url
        if ($LASTEXITCODE -ne 0) { throw 'Pinned model download failed' }
    }
    if ((Get-Item -LiteralPath $ModelFile).Length -ne $config.model.bytes -or
        (Get-FileHash -LiteralPath $ModelFile -Algorithm SHA256).Hash.ToLowerInvariant() -ne $config.model.sha256) {
        throw 'The complete pinned model failed size or checksum verification'
    }
    $app = Join-Path $repo 'target/x86_64-pc-windows-msvc/release/lugha-desk.exe'
    if ((Get-Item $app).VersionInfo.ProductVersion -ne $config.app_preview_version) { throw 'Unexpected app preview version' }
    $assets = Join-Path $buildRoot 'assets'
    & ./scripts/create-store-assets.ps1 -OutputDirectory $assets
    if ($LASTEXITCODE -ne 0) { throw 'Store graphics generation failed' }
    $payload = Join-Path $buildRoot 'payload'
    & $Python scripts/stage_store.py --app $app --runtime $RuntimeArchive --model $ModelFile --notices packaging/notices --assets $assets --output $payload --config packaging/store/store-config.json --framework $frameworkJson
    if ($LASTEXITCODE -ne 0) { throw 'Store payload validation failed' }
    $output = Join-Path $repo 'dist/store'
    if (Test-Path -LiteralPath $output) { throw 'Use a fresh Store output directory' }
    New-Item -ItemType Directory -Path $output | Out-Null
    $package = Join-Path $output 'Lugha-Desk-1.0.0.0-x64-STORE-SUBMISSION-UNSIGNED.msix'
    & $makeAppx pack /d $payload /p $package
    if ($LASTEXITCODE -ne 0) { throw 'Windows SDK MSIX validation/pack failed' }
    $unpacked = Join-Path $buildRoot 'unpacked'
    & $makeAppx unpack /p $package /d $unpacked
    if ($LASTEXITCODE -ne 0) { throw 'Windows SDK MSIX unpack failed' }
    & $Python scripts/verify_store_package.py --package $package --payload $payload
    if ($LASTEXITCODE -ne 0) { throw 'Actual MSIX integrity verification failed' }
    $sourceRevision = (& git rev-parse HEAD).Trim()
    if ($LASTEXITCODE -ne 0) { throw 'Cannot record exact source revision' }
    $report = [ordered]@{
        source_revision = $sourceRevision; app_preview_version = $config.app_preview_version
        package_version = $config.identity.version; store_id = $config.store_id
        windows_sdk = $sdk.Name; compiler_tools = $tools.Name; framework = $frameworkReport
        package_sha256 = (Get-FileHash $package -Algorithm SHA256).Hash.ToLowerInvariant()
        package_bytes = (Get-Item $package).Length; model_sha256 = $config.model.sha256
        signed = $false; published = $false; certification = 'not submitted'
        wack = 'not run'; packaged_lifecycle = 'not run'
    }
    $report | ConvertTo-Json -Depth 5 | Set-Content (Join-Path $output 'store-build-report.json') -Encoding utf8
    Copy-Item (Join-Path $payload 'store-payload.json'),$frameworkJson,packaging/store/listing.md,packaging/store/privacy.md,STORE_RELEASE.md $output
    ((Get-FileHash $package -Algorithm SHA256).Hash.ToLowerInvariant() + '  ' + (Split-Path $package -Leaf)) | Set-Content (Join-Path $output 'SHA256SUMS.txt') -Encoding ascii
    "Unsigned Store review package: $package"
} finally { Pop-Location }
