@echo off

cd %~dp0..\
cargo build -r

cd %~dp0
mkdir dist
copy ..\target\release\plugin.exe .\dist\main.exe
copy .\manifest.json .\dist\manifest.json
copy .\actions.json .\dist\actions.json

echo Done.
