cd ..\..
cargo build
target\debug\file_search.exe -g data\*.csv --search-expression tb_ --mode line-search
cd examples\ps1