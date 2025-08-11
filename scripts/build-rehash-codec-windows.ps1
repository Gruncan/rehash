$depPathRelative = "..\\.dependencies\\ffmpeg\\rehash-win64-dep"
$depPathAbsolute = (Resolve-Path $depPathRelative).Path

Write-Output $depPathAbsolute

$env:FFMPEG_DIR = $depPathAbsolute

cd ../components/rehash-codec/codec

cargo build

cd ../../../scripts