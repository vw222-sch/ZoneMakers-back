cargo build -r
copy target\release\*.exe .\bin\
copy db.sqlite .\bin\
copy readme.md .\bin\
rem tar -a -c -f bin/bin.zip .\bin