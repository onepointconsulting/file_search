cd ..\..
cargo build
target\debug\file_search.exe -g data\*.csv --search-expression "\b[jJ]im\b" --mode line-regex-search
cd examples\ps1