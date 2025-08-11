cd ../

$depUrl = "https://github.com/Gruncan/rehash/releases/download/ffmpeg-win64-dep/rehash-win64-dep.zip"

$downloadPath = "rehash-win64-dep.zip"

$depPath = "./.dependencies/ffmpeg"

if (-Not (Test-Path -Path $depPath))
{
    New-Item -ItemType Directory -Path $depPath
}

Invoke-WebRequest -Uri $depUrl -OutFile $downloadPath
Expand-Archive -Path $downloadPath -DestinationPath $depPath -Force

Write-Output "Downloaded win64 .dependencies!"

cd scripts