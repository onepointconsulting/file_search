cd ..\..
cargo build
target\debug\file_search.exe -g data\*.zip --search-expression tb_ --mode zip
cd batch\ps1