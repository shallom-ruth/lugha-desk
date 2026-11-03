param([Parameter(Mandatory=$true)][string]$OutputDirectory)
$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest
if (Test-Path -LiteralPath $OutputDirectory) { throw 'Use a fresh Store assets directory' }
Add-Type -AssemblyName System.Drawing
New-Item -ItemType Directory -Path $OutputDirectory -Force | Out-Null
foreach ($entry in @(@('StoreLogo.png',50), @('Square44x44Logo.png',44), @('Square150x150Logo.png',150))) {
    $side = [int]$entry[1]
    $bitmap = [Drawing.Bitmap]::new($side, $side, [Drawing.Imaging.PixelFormat]::Format32bppArgb)
    $graphics = [Drawing.Graphics]::FromImage($bitmap)
    $font = [Drawing.Font]::new('Segoe UI', [single]($side * 0.68), [Drawing.FontStyle]::Bold, [Drawing.GraphicsUnit]::Pixel)
    $brush = [Drawing.SolidBrush]::new([Drawing.ColorTranslator]::FromHtml('#F2F6F3'))
    $format = [Drawing.StringFormat]::new()
    try {
        $graphics.Clear([Drawing.ColorTranslator]::FromHtml('#123B34'))
        $graphics.TextRenderingHint = [Drawing.Text.TextRenderingHint]::AntiAliasGridFit
        $format.Alignment = [Drawing.StringAlignment]::Center
        $format.LineAlignment = [Drawing.StringAlignment]::Center
        $graphics.DrawString('L', $font, $brush, [Drawing.RectangleF]::new(0,0,$side,$side), $format)
        $bitmap.Save((Join-Path $OutputDirectory $entry[0]), [Drawing.Imaging.ImageFormat]::Png)
    } finally {
        $format.Dispose(); $brush.Dispose(); $font.Dispose(); $graphics.Dispose(); $bitmap.Dispose()
    }
}
'Generated Store app-name logos: 50, 44 and 150 pixels.'
