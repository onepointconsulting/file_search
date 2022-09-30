cd ..\..
cargo build
target\debug\file_search.exe -g data\*.zip --search-expression ".+event.+" --mode zip-regex
cd examples\ps1