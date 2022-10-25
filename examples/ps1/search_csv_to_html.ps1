cd ..\..
cargo build
target\debug\file_search.exe -g data\*.csv --search-expression tb_ --mode file-name `
--output html `
--file /tmp/search_csv.html
cd examples\ps1