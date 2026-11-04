param([Parameter(Mandatory)][string]$ArtifactDirectory, [Parameter(Mandatory)][string]$OutputDirectory,
    [switch]$ExerciseRemovalFailure)
$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest
# Test certificates and package deployment are confined to an ephemeral hosted runner.
if ($env:GITHUB_ACTIONS -ne 'true' -or $env:RUNNER_ENVIRONMENT -ne 'github-hosted') {
    throw 'Run this test only in the disposable GitHub-hosted validation job'
}
$ArtifactDirectory = (Resolve-Path $ArtifactDirectory).Path
$OutputDirectory = (Resolve-Path $OutputDirectory).Path
$report = [ordered]@{ install='not run'; activation='not run'; packaged_cli='not run'; uninstall='not run'; wack='not run'
    update='not run'; gui_cancel_reopen='not run'; exe_coexistence='not run'
    limitation='Windows 11 ARM runner tests the x64 package under emulation; native x64 hardware and manual lifecycle checks remain required.' }
$certificate = $null
$installed = $null
$rules = @()
$removalFailure = $null
$standalone = $null
. (Join-Path $PSScriptRoot '../tests/store_lifecycle_helpers.ps1')
try {
    if (@(Get-AppxPackage RuthShallom.LughaDesk).Count) { throw 'Refusing to modify an existing Lugha Desk installation' }
    $package = @(Get-ChildItem $ArtifactDirectory -Filter '*.msix' -File)
    if ($package.Count -ne 1) { throw 'Expected one unsigned Store package' }
    $build = Get-Content (Join-Path $ArtifactDirectory 'store-build-report.json') -Raw | ConvertFrom-Json
    if ((Get-FileHash $package[0].FullName -Algorithm SHA256).Hash.ToLowerInvariant() -ne $build.package_sha256) { throw 'Package checksum mismatch' }
    $sdkRoot = Join-Path ${env:ProgramFiles(x86)} 'Windows Kits/10'
    $signTool = Get-ChildItem $sdkRoot -Filter signtool.exe -Recurse |
        Where-Object { $_.FullName -match '[\\/]x64[\\/]' } | Sort-Object FullName -Descending | Select-Object -First 1
    if (-not $signTool) { throw 'Windows SDK x64 SignTool is unavailable in this client runner' }
    $frameworkRoot = Join-Path ${env:ProgramFiles(x86)} 'Microsoft SDKs/Windows Kits/10/ExtensionSDKs/Microsoft.VCLibs.Desktop'
    $framework = Get-ChildItem $frameworkRoot -Recurse -Filter '*.appx' -ErrorAction SilentlyContinue |
        Where-Object { $_.FullName -match '[\\/]Appx[\\/]Retail[\\/]x64[\\/]' } | Sort-Object FullName -Descending | Select-Object -First 1
    if (-not $framework) {
        $frameworkPath = Join-Path $env:RUNNER_TEMP 'Microsoft.VCLibs.x64.Desktop.appx'
        & curl.exe --fail --location --silent --show-error --retry 3 --proto '=https' --proto-redir '=https' --output $frameworkPath 'https://aka.ms/Microsoft.VCLibs.x64.14.00.Desktop.appx'
        if ($LASTEXITCODE -ne 0) { throw 'Official Desktop framework download failed' }
    } else { $frameworkPath = $framework.FullName }
    $signature = Get-AuthenticodeSignature $frameworkPath
    if ($signature.Status -ne 'Valid' -or $signature.SignerCertificate.Subject -notlike '*O=Microsoft Corporation*') { throw 'Framework Microsoft signature validation failed' }
    $testCopy = Join-Path $env:RUNNER_TEMP 'Lugha-Desk-TEST-ONLY.msix'
    Copy-Item $package[0].FullName $testCopy
    $certificate = New-SelfSignedCertificate -Type Custom -Subject 'CN=417167AF-8330-486F-8BD2-5C96DFDA0EE8' -KeyUsage DigitalSignature -FriendlyName 'Ephemeral Lugha CI test only' -CertStoreLocation Cert:\CurrentUser\My -TextExtension @('2.5.29.37={text}1.3.6.1.5.5.7.3.3','2.5.29.19={text}')
    # Only the public certificate is exported; no PFX or private key is published.
    $publicCert = Join-Path $env:RUNNER_TEMP 'Lugha-Desk-TEST-ONLY.cer'
    Export-Certificate -Cert $certificate -FilePath $publicCert | Out-Null
    Import-Certificate -FilePath $publicCert -CertStoreLocation Cert:\LocalMachine\TrustedPeople | Out-Null
    & $signTool.FullName sign /fd SHA256 /sha1 $certificate.Thumbprint /s My $testCopy
    if ($LASTEXITCODE -ne 0) { throw 'Ephemeral test signing failed' }
    Add-AppxPackage -Path $testCopy -DependencyPath $frameworkPath
    $installed = Get-AppxPackage RuthShallom.LughaDesk
    if ($installed.PackageFamilyName -ne 'RuthShallom.LughaDesk_mj1zfg7cgz4xj' -or $installed.Version.ToString() -ne '1.0.1.0') { throw 'Installed identity mismatch' }
    $report.install = 'passed'
    $report.package_source_revision = $build.source_revision
    $report.original_unsigned_sha256 = $build.package_sha256
    $report.install_location = $installed.InstallLocation
    if ($ExerciseRemovalFailure) { return } # Exercise the real still-installed postcondition below.
    $standalone = New-StandaloneStoreFixture $installed $OutputDirectory
    $report.exe_coexistence = 'standalone EXE/model fixture in Unicode/space path translated while Store package installed; preservation after uninstall pending'
    # Use Windows' documented Internet scope, preserving the local worker connection.
    foreach ($program in @((Join-Path $installed.InstallLocation 'lugha-desk.exe'),(Join-Path $installed.InstallLocation 'runtime/llama-server.exe'))) {
        $ruleName = 'Lugha-CI-offline-' + [guid]::NewGuid().ToString('N')
        New-NetFirewallRule -Name $ruleName -DisplayName $ruleName -Direction Outbound -Action Block -Program $program -RemoteAddress Internet | Out-Null
        $rules += $ruleName
    }
    $report.network_test_scope = 'Windows Firewall Internet keyword for installed app and server; LAN remains available. Not physical network disconnection.'
    $driver = Join-Path $PSScriptRoot 'test-store-context.ps1'
    Invoke-CommandInDesktopPackage -PackageFamilyName $installed.PackageFamilyName -AppId LughaDesk -Command "$env:SystemRoot\System32\WindowsPowerShell\v1.0\powershell.exe" -Args "-NoProfile -ExecutionPolicy Bypass -File `"$driver`" -InstallLocation `"$($installed.InstallLocation)`" -OutputDirectory `"$OutputDirectory`"" -PreventBreakaway
    $contextResult = Join-Path $OutputDirectory 'package-context.json'
    $deadline = (Get-Date).AddMinutes(12)
    while (-not (Test-Path $contextResult) -and (Get-Date) -lt $deadline) { Start-Sleep -Seconds 2 }
    if (-not (Test-Path $contextResult)) { throw 'Packaged context test did not return a report' }
    $context = Get-Content $contextResult -Raw | ConvertFrom-Json
    if ($context.status -ne 'passed') { throw "Packaged CLI test failed: $($context.error)" }
    $report.packaged_cli = 'passed with Internet traffic blocked; outputs require language review'
    # Direct user activation is separate from the debugging-context CLI checks.
    Start-Process explorer.exe -ArgumentList "shell:AppsFolder\$($installed.PackageFamilyName)!LughaDesk"
    $deadline = (Get-Date).AddSeconds(30)
    $appProcess = $null
    while (-not $appProcess -and (Get-Date) -lt $deadline) {
        Start-Sleep -Seconds 1
        $appProcess = Get-Process lugha-desk -ErrorAction SilentlyContinue | Where-Object { $_.Path -eq (Join-Path $installed.InstallLocation 'lugha-desk.exe') } | Select-Object -First 1
    }
    if (-not $appProcess) { throw 'Direct Start-menu activation did not create the installed app process' }
    $report.activation = 'installed executable process observed; visual GUI checks still required'
    $appProcess.CloseMainWindow() | Out-Null
    if (-not $appProcess.WaitForExit(10000)) { $appProcess.Kill() }
    $wack = Get-ChildItem $sdkRoot -Filter appcert.exe -Recurse | Select-Object -First 1
    if ($wack) {
        & $wack.FullName test -appxpackagepath $testCopy -reportoutputpath (Join-Path $OutputDirectory 'wack.xml')
        $report.wack_exit_code = $LASTEXITCODE
        $report.wack = 'executed; inspect the complete XML report, exit code alone is not certification'
    } else { $report.wack = 'blocked: WACK is not installed on this runner' }
    $installed = Install-StoreTestUpdate $package[0].FullName $certificate $signTool.FullName $frameworkPath $OutputDirectory
    $updatedModel = Join-Path $installed.InstallLocation 'model/TranslatePsy-AfriSLM-0.8B-Q8_0-imat.gguf'
    if ((Get-FileHash $updatedModel -Algorithm SHA256).Hash.ToLowerInvariant() -ne $build.model_sha256) { throw 'Updated package model hash mismatch' }
    $updateOutput = Join-Path $OutputDirectory 'updated-package'
    New-Item -ItemType Directory $updateOutput | Out-Null
    Invoke-CommandInDesktopPackage -PackageFamilyName $installed.PackageFamilyName -AppId LughaDesk -Command "$env:SystemRoot\System32\WindowsPowerShell\v1.0\powershell.exe" -Args "-NoProfile -ExecutionPolicy Bypass -File `"$driver`" -InstallLocation `"$($installed.InstallLocation)`" -OutputDirectory `"$updateOutput`" -ExpectedVersion 1.0.1.1" -PreventBreakaway
    $updatedResult = Join-Path $updateOutput 'package-context.json'
    $deadline = (Get-Date).AddMinutes(12)
    while (-not (Test-Path $updatedResult) -and (Get-Date) -lt $deadline) { Start-Sleep -Seconds 2 }
    if (-not (Test-Path $updatedResult)) { throw 'Updated package translation timed out' }
    if ((Get-Content $updatedResult -Raw | ConvertFrom-Json).status -ne 'passed') { throw 'Updated package translation failed' }
    $report.update = 'test-only 1.0.1.0 to 1.0.1.1 deployment and three packaged CLI cases passed; production artifact unchanged'
    if ((Get-FileHash $package[0].FullName -Algorithm SHA256).Hash.ToLowerInvariant() -ne $build.package_sha256) { throw 'Original unsigned artifact changed during lifecycle tests' }
} catch {
    $report.error = $_.Exception.Message
    throw
} finally {
    foreach ($rule in $rules) { Remove-NetFirewallRule -Name $rule -ErrorAction SilentlyContinue }
    if ($installed) {
        try {
            if (-not $ExerciseRemovalFailure) { Remove-AppxPackage -Package $installed.PackageFullName }
            if (@(Get-AppxPackage RuthShallom.LughaDesk).Count) { throw 'Package remains after removal' }
            $report.uninstall = 'passed'
        } catch {
            $report.uninstall = 'failed: ' + $_.Exception.Message
            $removalFailure = $_.Exception
            # Clean the deliberately retained package after observing the failure.
            if ($ExerciseRemovalFailure) { Remove-AppxPackage -Package $installed.PackageFullName }
        }
    }
    if ($certificate) {
        Remove-Item "Cert:\LocalMachine\TrustedPeople\$($certificate.Thumbprint)" -ErrorAction SilentlyContinue
        Remove-Item "Cert:\CurrentUser\My\$($certificate.Thumbprint)" -ErrorAction SilentlyContinue
    }
    if ($standalone) {
        try {
            Assert-StandaloneStoreFixture $standalone $OutputDirectory
            $report.exe_coexistence = 'standalone EXE/model fixture in Unicode/space path unchanged by Store update/uninstall; translation passed before and after. Not a GitHub launcher upgrade test.'
        } catch {
            $report.exe_coexistence = 'failed: ' + $_.Exception.Message
            $removalFailure = $_.Exception
        }
    }
    $report | ConvertTo-Json -Depth 5 | Set-Content (Join-Path $OutputDirectory 'package-test-report.json') -Encoding utf8
    # Finish certificate cleanup and evidence writing, then fail the CI process.
    if ($removalFailure) { throw $removalFailure }
}
