@echo off
cd /d "%~dp0"
powershell -NoProfile -ExecutionPolicy Bypass -Command ^
"$prefix = 'concatenated_part'; ^
$targetDirs = @('examples', 'src'); ^
$maxBytes = 1500000; ^
$root = (Get-Location).Path; ^
$part = 1; ^
$currentSize = 0; ^
$outFile = Join-Path $root \"$prefix$part.txt\"; ^
$utf8NoBom = New-Object System.Text.UTF8Encoding $false; ^
$writer = [System.IO.StreamWriter]::new($outFile, $false, $utf8NoBom); ^
$skipExt = @('.exe','.dll','.png','.jpg','.jpeg','.gif','.ico','.zip','.pdf','.bin','.woff','.woff2','.ttf','.eot','.mp3','.mp4','.webm'); ^
foreach ($dir in $targetDirs) { ^
    $fullDir = Join-Path $root $dir; ^
    if (-not (Test-Path $fullDir)) { continue } ^
    $files = Get-ChildItem -Path $fullDir -Recurse -File; ^
    foreach ($file in $files) { ^
        if ($skipExt -contains $file.Extension.ToLower()) { continue } ^
        $relPath = $file.FullName.Substring($root.Length + 1).Replace('\', '/'); ^
        $mid = \"===== $relPath =======\"; ^
        $bar = '=' * $mid.Length; ^
        $header = \"`r`n$bar`r`n$mid`r`n$bar`r`n\"; ^
        try { $content = [System.IO.File]::ReadAllText($file.FullName) } catch { continue } ^
        $block = $header + $content + \"`r`n\"; ^
        $blockBytes = $utf8NoBom.GetByteCount($block); ^
        if (($currentSize + $blockBytes) -gt $maxBytes -and $currentSize -gt 0) { ^
            $writer.Close(); ^
            $part++; ^
            $outFile = Join-Path $root \"$prefix$part.txt\"; ^
            $writer = [System.IO.StreamWriter]::new($outFile, $false, $utf8NoBom); ^
            $currentSize = 0; ^
        } ^
        $writer.Write($block); ^
        $currentSize += $blockBytes; ^
    } ^
} ^
$writer.Close(); ^
Write-Host \"Done! Created $part file(s) starting with '$prefix'.\" -ForegroundColor Green"
pause
