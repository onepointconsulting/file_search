cd ..\..
cargo build
target\debug\file_search.exe -g data\*.csv --search-expression oslo --mode line-search
cd examples\ps1