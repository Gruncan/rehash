cd ../

$depUrl = "https://github.com/Gruncan/rehash/releases/download/ffmpeg-win64-dep/rehash-win64-dep.zip"
$downloadPath = "rehash-win64-dep.zip"
$depPath = "./.dependencies/ffmpeg"
$binPath = Join-Path $depPath "rehash-win64-dep/bin/*"
$binLoc = "rehash-desktop/codec"

if (-Not (Test-Path -Path $depPath))
{
    New-Item -ItemType Directory -Path $depPath
}
Clear-Host

Invoke-WebRequest -Uri $depUrl -OutFile $downloadPath
Expand-Archive -Path $downloadPath -DestinationPath $depPath -Force

Clear-Host
Write-Output "Downloaded win64 dependencies!"


Move-Item -Path $binPath -Destination $binLoc

Write-Output "Moved binaries for rehash desktop build!"

cd scripts