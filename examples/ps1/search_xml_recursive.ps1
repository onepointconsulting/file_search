cd ..\..
cargo build
target\debug\file_search.exe -g C:\development\onepoint\autolus\git2\Autolus\autolus-datalake-app\**\*.xml --search-expression smtp.user --mode line-search
cd examples\ps1