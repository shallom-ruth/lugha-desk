# Deployment tests only: original review artifact is never modified or uploaded signed.
if ($env:GITHUB_ACTIONS -ne 'true' -or $env:RUNNER_ENVIRONMENT -ne 'github-hosted') {
    throw 'Disposable hosted runner only'
}
function Invoke-StandaloneStoreSmoke([string]$Directory, [string]$OutputDirectory) {
    $start = [Diagnostics.ProcessStartInfo]::new()
    $start.FileName = Join-Path $Directory 'lugha-desk.exe'
    $start.Arguments = '--translate en-sw'
    $start.UseShellExecute = $false
    $start.CreateNoWindow = $true
    $start.RedirectStandardInput = $true
    $start.RedirectStandardOutput = $true
    $start.RedirectStandardError = $true
    $start.StandardOutputEncoding = [Text.UTF8Encoding]::new($false)
    $process = [Diagnostics.Process]::Start($start)
    try {
        $stdout = $process.StandardOutput.ReadToEndAsync()
        $stderr = $process.StandardError.ReadToEndAsync()
        $bytes = [Text.Encoding]::UTF8.GetBytes('The child reads a book.')
        $process.StandardInput.BaseStream.Write($bytes,0,$bytes.Length)
        $process.StandardInput.Close()
        if (-not $process.WaitForExit(180000)) { $process.Kill(); throw 'Standalone translation timed out' }
        $text = $stdout.GetAwaiter().GetResult()
        $errorText = $stderr.GetAwaiter().GetResult()
        if ($process.ExitCode -ne 0 -or [string]::IsNullOrWhiteSpace($text)) { throw "Standalone translation failed: $errorText" }
        [ordered]@{ input='The child reads a book.'; output=$text; exit_code=$process.ExitCode } |
            ConvertTo-Json | Set-Content (Join-Path $OutputDirectory 'standalone-smoke.json') -Encoding utf8
    } finally { $process.Dispose() }
}
function New-StandaloneStoreFixture($Installed, [string]$OutputDirectory) {
    $directory = Join-Path $env:RUNNER_TEMP ('Lugha coexistence é ' + [guid]::NewGuid().ToString('N'))
    New-Item -ItemType Directory $directory | Out-Null
    Copy-Item (Join-Path $Installed.InstallLocation 'lugha-desk.exe') $directory
    foreach ($name in @('runtime','model')) { Copy-Item (Join-Path $Installed.InstallLocation $name) $directory -Recurse }
    # The standalone fixture needs the same Microsoft VC dependency, app-local only.
    $framework = Get-AppxPackage Microsoft.VCLibs.140.00.UWPDesktop |
        Where-Object { $_.Architecture.ToString() -eq 'X64' } | Sort-Object Version -Descending | Select-Object -First 1
    if (-not $framework) { throw 'Installed x64 framework missing' }
    $dlls = @(Get-ChildItem $framework.InstallLocation -File | Where-Object { $_.Name -match '^(vcruntime140|msvcp140|concrt140|vcomp140|vccorlib140|vcamp140).*\.dll$' })
    foreach ($dll in $dlls) {
        Copy-Item $dll.FullName $directory
        Copy-Item $dll.FullName (Join-Path $directory 'runtime')
    }
    $hashes = @{}
    foreach ($file in Get-ChildItem $directory -File -Recurse) {
        $relative = $file.FullName.Substring($directory.Length + 1)
        $hashes[$relative] = (Get-FileHash $file.FullName -Algorithm SHA256).Hash
    }
    Invoke-StandaloneStoreSmoke $directory $OutputDirectory
    return @{ directory=$directory; hashes=$hashes }
}
function Assert-StandaloneStoreFixture($Fixture, [string]$OutputDirectory) {
    $files = @(Get-ChildItem $Fixture.directory -File -Recurse)
    if ($files.Count -ne $Fixture.hashes.Count) { throw 'Standalone file set changed during Store lifecycle' }
    foreach ($relative in $Fixture.hashes.Keys) {
        if ((Get-FileHash (Join-Path $Fixture.directory $relative) -Algorithm SHA256).Hash -ne $Fixture.hashes[$relative]) {
            throw "Standalone file changed: $relative"
        }
    }
    Invoke-StandaloneStoreSmoke $Fixture.directory $OutputDirectory
}
function Install-StoreTestUpdate([string]$Package, $Certificate, [string]$SignTool, [string]$FrameworkPath, [string]$OutputDirectory) {
    $makeappx = Join-Path (Split-Path $SignTool -Parent) 'makeappx.exe'
    if (-not (Test-Path $makeappx)) { throw 'SDK MakeAppx missing for update test' }
    $unpack = Join-Path $env:RUNNER_TEMP ('Lugha-update-' + [guid]::NewGuid().ToString('N'))
    & $makeappx unpack /p $Package /d $unpack /o | Out-Host
    if ($LASTEXITCODE -ne 0) { throw 'Update fixture unpack failed' }
    $manifestPath = Join-Path $unpack 'AppxManifest.xml'
    [xml]$manifest = Get-Content $manifestPath -Raw
    if ($manifest.Package.Identity.Version -ne '1.0.0.0') { throw 'Unexpected original version' }
    $manifest.Package.Identity.Version = '1.0.0.1'
    $manifest.Save($manifestPath)
    $updated = Join-Path $env:RUNNER_TEMP 'Lugha-Desk-UPDATE-TEST-ONLY.msix'
    & $makeappx pack /d $unpack /p $updated /o | Out-Host
    if ($LASTEXITCODE -ne 0) { throw 'Update fixture pack failed' }
    & $SignTool sign /fd SHA256 /sha1 $Certificate.Thumbprint /s My $updated | Out-Host
    if ($LASTEXITCODE -ne 0) { throw 'Update fixture signing failed' }
    Add-AppxPackage -Path $updated -DependencyPath $FrameworkPath
    $installed = Get-AppxPackage RuthShallom.LughaDesk
    if ($installed.Version.ToString() -ne '1.0.0.1' -or $installed.PackageFamilyName -ne 'RuthShallom.LughaDesk_mj1zfg7cgz4xj') { throw 'Store test update identity mismatch' }
    return $installed
}
