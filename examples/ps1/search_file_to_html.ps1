cd ..\..
cargo build
target\debug\file_search.exe -g data\*.csv --search-expression "\b[jJ]im\b" --mode line-regex-search --output html --file /tmp/search_line_res.html
cd examples\ps1