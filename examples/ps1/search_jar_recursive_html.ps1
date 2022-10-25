cd ..\..
cargo build
target\debug\file_search.exe -g C:\Users\gilfe\.m2\repository\**\*.jar `
--search-expression org/glassfish/jersey/client/internal/LocalizationMessages `
--mode zip `
--output html `
--file /tmp/search_jar_recursive.html
cd examples\ps1