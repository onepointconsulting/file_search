cd ..\..
cargo build
target\debug\file_search.exe -g data/*.json --search-expression "$..name" --mode json-path
cd examples\ps1