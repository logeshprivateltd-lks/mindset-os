Add-Type -AssemblyName System.Drawing

$dir = "C:\MINDSETOS\apps\mindset-desktop\src-tauri\icons"
if (-not (Test-Path $dir)) {
    New-Item -ItemType Directory -Path $dir -Force | Out-Null
}

function Create-PngFile {
    param([int]$size, [string]$filePath)
    $bmp = New-Object System.Drawing.Bitmap $size, $size
    $g = [System.Drawing.Graphics]::FromImage($bmp)
    $g.SmoothingMode = [System.Drawing.Drawing2D.SmoothingMode]::AntiAlias
    $brush = New-Object System.Drawing.SolidBrush ([System.Drawing.ColorTranslator]::FromHtml("#8B9CFF"))
    $g.FillEllipse($brush, 2, 2, $size - 4, $size - 4)
    $innerBrush = New-Object System.Drawing.SolidBrush ([System.Drawing.ColorTranslator]::FromHtml("#07090D"))
    $g.FillEllipse($innerBrush, [int]($size * 0.25), [int]($size * 0.25), [int]($size * 0.5), [int]($size * 0.5))
    $g.Dispose()
    $bmp.Save($filePath, [System.Drawing.Imaging.ImageFormat]::Png)
    $bmp.Dispose()
}

Create-PngFile 32 (Join-Path $dir "32x32.png")
Create-PngFile 128 (Join-Path $dir "128x128.png")
Create-PngFile 256 (Join-Path $dir "128x128@2x.png")

# Create icon.ico
$bmp32 = New-Object System.Drawing.Bitmap 32, 32
$g32 = [System.Drawing.Graphics]::FromImage($bmp32)
$brush32 = New-Object System.Drawing.SolidBrush ([System.Drawing.ColorTranslator]::FromHtml("#8B9CFF"))
$g32.FillEllipse($brush32, 1, 1, 30, 30)
$g32.Dispose()
$hIcon = $bmp32.GetHicon()
$icon = [System.Drawing.Icon]::FromHandle($hIcon)
$fs = New-Object System.IO.FileStream (Join-Path $dir "icon.ico"), ([System.IO.FileMode]::Create)
$icon.Save($fs)
$fs.Close()
$bmp32.Dispose()

# Create dummy icon.icns
[System.IO.File]::WriteAllBytes((Join-Path $dir "icon.icns"), (New-Object byte[] 64))

Write-Output "Icons generated successfully in $dir"

