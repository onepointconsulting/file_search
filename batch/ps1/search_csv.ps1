cd ..\..
cargo build
target\debug\file_search.exe -g data\*.csv --search-expression tb_ --mode file-name
cd batch\ps1