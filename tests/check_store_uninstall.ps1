param([Parameter(Mandatory)][string]$ArtifactDirectory,[Parameter(Mandatory)][string]$OutputDirectory)
$ErrorActionPreference = 'Stop'
if ($env:GITHUB_ACTIONS -ne 'true' -or $env:RUNNER_ENVIRONMENT -ne 'github-hosted') { throw 'Disposable hosted runner only' }
$failureOutput = Join-Path $OutputDirectory 'uninstall-failure-regression'
New-Item -ItemType Directory $failureOutput -Force | Out-Null
$testScript = Join-Path (Split-Path $PSScriptRoot -Parent) 'scripts/test-store-install.ps1'
$shell = (Get-Process -Id $PID).Path
& $shell -NoProfile -File $testScript -ArtifactDirectory $ArtifactDirectory -OutputDirectory $failureOutput -ExerciseRemovalFailure
$childExit = $LASTEXITCODE
$observed = Get-Content (Join-Path $failureOutput 'package-test-report.json') -Raw | ConvertFrom-Json
if ($observed.install -ne 'passed' -or $observed.uninstall -notlike 'failed:*') { throw 'Did not exercise the installed-package removal postcondition' }
if (@(Get-AppxPackage RuthShallom.LughaDesk).Count) { throw 'Failure-path test did not clean its disposable package' }
if ($childExit -eq 0) { throw 'Uninstall failure incorrectly returned process success' }
'PASS: real retained-package failure was reported, returned nonzero, and was cleaned up.'
