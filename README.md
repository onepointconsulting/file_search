## Rust File Search utility

Simple binary programme used to grep files by name, or for searching inside of compressed files.

### Usage Instructions

```
file_search
Simple binary programme used to grep files by name, or for searching inside of compressed files.
JSON path is also supported when using json-path mode

USAGE:
    file_search.exe [OPTIONS] --glob-pattern <GLOB_PATTERN> --mode <MODE>

OPTIONS:
    -f, --file <FILE>
            The output file in case the output parameter is "File"

    -g, --glob-pattern <GLOB_PATTERN>
            The glob pattern used to list files, e.g. *.zip or /media/**/*.csv

    -h, --help
            Print help information

    -m, --mode <MODE>
            The operation mode [possible values: file-name, zip, line-search, line-regex-search,
            zip-regex, json-path]

    -o, --output <OUTPUT>
            The output mode [possible values: console, file]

    -s, --search-expression <SEARCH_EXPRESSION>
            The search expression, like 'foo' or if using json path e.g. '$..name'. Not a regular
            expression unless you should use "line-search-regex"

```

### Examples

Searching for csv file names with the `tb_` substring:

```ps1
file_search.exe -g data\*.csv --search-expression tb_ --mode file-name
```

