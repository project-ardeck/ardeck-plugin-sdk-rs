@echo off

cargo build -r
cd %~dp0
mkdir dist
copy ..\target\release\ardeck-plugin.exe .\dist\main.exe
copy .\manifest.json .\dist\manifest.json
copy .\actions.json .\dist\actions.json

echo Done.
