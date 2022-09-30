cd ..\..
cargo build
target\debug\file_search.exe -g data\*.csv --mode file-name
cd examples\ps1