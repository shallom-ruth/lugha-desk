param(
    [string]$RuntimeArchive,
    [string]$Python = 'python'
)
$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest
$repo = Split-Path $PSScriptRoot -Parent
Push-Location $repo
try {
    & $Python tests/test_stage_bundle.py
    if ($LASTEXITCODE -ne 0) { throw 'Packaging tests failed' }
    & cargo test --locked
    if ($LASTEXITCODE -ne 0) { throw 'App tests failed' }
    & cargo build --release --locked --target x86_64-pc-windows-msvc
    if ($LASTEXITCODE -ne 0) { throw 'App build failed' }
    & $Python tests/test_windows_manifest.py target/x86_64-pc-windows-msvc/release/lugha-desk.exe
    if ($LASTEXITCODE -ne 0) { throw 'Actual EXE DPI/security manifest failed' }
    & $Python tests/test_greetings.py target/x86_64-pc-windows-msvc/release/lugha-desk.exe
    if ($LASTEXITCODE -ne 0) { throw 'Greeting integration checks failed' }
    $lock = Get-Content scripts/runtime-lock.json -Raw | ConvertFrom-Json
    if (-not $RuntimeArchive) {
        New-Item -ItemType Directory -Path 'target/vendor' -Force | Out-Null
        $RuntimeArchive = Join-Path $repo 'target/vendor/llama-runtime.zip'
        Invoke-WebRequest -Uri $lock.url -OutFile $RuntimeArchive
    }
    $stage = Join-Path $repo ('target/staging-' + [guid]::NewGuid().ToString('N'))
    & $Python scripts/stage_bundle.py --app target/x86_64-pc-windows-msvc/release/lugha-desk.exe --runtime $RuntimeArchive --lock scripts/runtime-lock.json --notices packaging/notices --output $stage
    if ($LASTEXITCODE -ne 0) { throw 'Staging failed' }
    $previousAssetRoot = $env:LUGHA_ASSET_ROOT
    try {
        $env:LUGHA_ASSET_ROOT = $stage
        & $Python launcher/pack_assets.py
        if ($LASTEXITCODE -ne 0) { throw 'Resource packaging failed' }
    } finally { $env:LUGHA_ASSET_ROOT = $previousAssetRoot }
    & cargo test --locked --manifest-path launcher/Cargo.toml
    if ($LASTEXITCODE -ne 0) { throw 'Launcher tests failed' }
    & cargo build --release --locked --target x86_64-pc-windows-msvc --manifest-path launcher/Cargo.toml
    if ($LASTEXITCODE -ne 0) { throw 'Launcher build failed' }
    $version = (& $Python -c "import tomllib; print(tomllib.load(open('Cargo.toml','rb'))['package']['version'])").Trim()
    if ($LASTEXITCODE -ne 0) { throw 'Could not read release version' }
    $output = Join-Path $repo ('dist/unsigned-' + $version)
    if (Test-Path $output) { throw 'Output directory already exists; use a fresh checkout' }
    New-Item -ItemType Directory -Path $output | Out-Null
    Copy-Item 'target/x86_64-pc-windows-msvc/release/lugha-desk.exe' $output
    Copy-Item 'launcher/target/x86_64-pc-windows-msvc/release/lugha-desk-launcher.exe' $output
    Copy-Item 'CODE_SIGNING.md','INSTALLATION.md' $output
    foreach ($name in @('lugha-desk.exe','lugha-desk-launcher.exe')) {
        $file = Join-Path $output $name
        $info = (Get-Item $file).VersionInfo
        if ($info.ProductName -ne 'Lugha Desk' -or $info.ProductVersion -ne $version -or $info.OriginalFilename -ne $name) { throw "Invalid PE metadata: $name" }
        $signature = Get-AuthenticodeSignature $file
        if ($signature.Status -ne 'NotSigned') { throw "Expected unsigned review artifact: $name" }
    }
    $hashes = Get-ChildItem $output -Filter '*.exe' | Sort-Object Name | ForEach-Object {
        ((Get-FileHash $_.FullName -Algorithm SHA256).Hash.ToLowerInvariant()) + '  ' + $_.Name
    }
    $hashes | Set-Content (Join-Path $output 'SHA256SUMS.txt') -Encoding ascii
    'UNSIGNED REVIEW BUILD. No signing certificate has been issued; see CODE_SIGNING.md for current status.' | Set-Content (Join-Path $output 'UNSIGNED.txt')
    Write-Output "Unsigned review artifacts: $output"
} finally { Pop-Location }
