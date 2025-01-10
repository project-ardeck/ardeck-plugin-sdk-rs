@echo off

cargo build -r

mkdir dist
copy .\target\release\ardeck-plugin-sdk-rs.exe .\dist\main.exe
copy .\manifest.json .\dist\manifest.json
copy .\actions.json .\dist\actions.json

pause
