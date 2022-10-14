cd ..\..
cargo build
target\debug\file_search.exe -g data\*.csv --mode file-name --output html --file /tmp/search_csv_no_expression.html
cd examples\ps1