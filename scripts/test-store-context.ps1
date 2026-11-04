param([Parameter(Mandatory)][string]$InstallLocation,[Parameter(Mandatory)][string]$OutputDirectory)
$ErrorActionPreference = 'Stop'
$result = [ordered]@{ status='failed'; context='debug process with package identity, not full GUI parity'; cases=@() }
try {
    Add-Type @'
using System;
using System.Text;
using System.Runtime.InteropServices;
public static class PackageIdentity {
    [DllImport("kernel32.dll", CharSet=CharSet.Unicode)]
    public static extern int GetCurrentPackageFullName(ref uint length, StringBuilder name);
    public static string Current() {
        uint length = 0;
        int error = GetCurrentPackageFullName(ref length, null);
        if (error != 122) throw new Exception("No package identity: " + error);
        var name = new StringBuilder((int)length);
        error = GetCurrentPackageFullName(ref length, name);
        if (error != 0) throw new Exception("Package identity failed: " + error);
        return name.ToString();
    }
}
'@
    $result.package_identity = [PackageIdentity]::Current()
    if ($result.package_identity -notlike 'RuthShallom.LughaDesk_1.0.0.0_x64__mj1zfg7cgz4xj') { throw 'Unexpected debug process package identity' }
    foreach ($case in @(@('en-sw','Good morning.'),@('en-sw','The child reads a book.'),@('sw-en','Mtoto anasoma kitabu.'))) {
        $start = New-Object Diagnostics.ProcessStartInfo
        $start.FileName = Join-Path $InstallLocation 'lugha-desk.exe'
        $start.Arguments = '--translate ' + $case[0]
        $start.UseShellExecute = $false
        $start.CreateNoWindow = $true
        $start.RedirectStandardInput = $true
        $start.RedirectStandardOutput = $true
        $start.RedirectStandardError = $true
        $start.StandardInputEncoding = New-Object Text.UTF8Encoding($false)
        $start.StandardOutputEncoding = New-Object Text.UTF8Encoding($false)
        $process = [Diagnostics.Process]::Start($start)
        $output = $process.StandardOutput.ReadToEndAsync()
        $errorOutput = $process.StandardError.ReadToEndAsync()
        $process.StandardInput.Write($case[1])
        $process.StandardInput.Close()
        if (-not $process.WaitForExit(180000)) { $process.Kill(); throw 'Installed translation timed out' }
        $text = $output.GetAwaiter().GetResult()
        $errorText = $errorOutput.GetAwaiter().GetResult()
        if ($process.ExitCode -ne 0 -or [string]::IsNullOrWhiteSpace($text)) { throw "Installed translation failed: $errorText" }
        $result.cases += [ordered]@{ direction=$case[0]; input=$case[1]; output=$text; exit_code=$process.ExitCode }
        $process.Dispose()
    }
    $result.status = 'passed'
} catch { $result.error = $_.Exception.Message }
$result | ConvertTo-Json -Depth 5 | Set-Content (Join-Path $OutputDirectory 'package-context.json') -Encoding utf8
