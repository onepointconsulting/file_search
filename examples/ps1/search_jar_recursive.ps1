cd ..\..
cargo build
target\debug\file_search.exe -g C:\Users\gilfe\.m2\repository\**\*.jar --search-expression org/glassfish/jersey/client/internal/LocalizationMessages --mode zip
cd examples\ps1