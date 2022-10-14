## Rust File Search utility

Simple binary programme used to grep files by name, or for searching inside of compressed files.

### Usage Instructions

```
file_search 
Simple binary programme used to grep files by name, or for searching inside of compressed files.
JSON path is also supported when using json-path mode. If using json path, it should be a valid json
path expression

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
            zip-regex, json-path, pdf-search]

    -o, --output <OUTPUT>
            The output mode [possible values: console, file, html]

    -s, --search-expression <SEARCH_EXPRESSION>
            The search expression, like 'foo' or if using json path e.g. '$..name'. Not a regular
            expression unless you should use "line-search-regex"

```

### Build

```ps1
cargo build -r
```

### Examples

Searching just for json files recursively:

```ps1
file_search.exe -g \tmp\**\*.json --mode file-name
```

Searching for csv file names with the `tb_` substring:

```ps1
file_search.exe -g data\*.csv --search-expression tb_ --mode file-name
```

Searching recursively for json files and searching in those files for a specific JSON path:

```ps1
file_search.exe -g \tmp\**\*.json -m json-path -s "$.newCustomer.customerId.masterKey.systemOwner"
```

Searching in jar files using recursion for certain classes:

```ps1
file_search.exe -g C:\Users\gilfe\.m2\repository\**\*.jar --search-expression org/glassfish/jersey/client/internal/LocalizationMessages --mode zip
```

Searching in csv files for a simple pattern:

```ps1
file_search.exe -g data\*.csv --search-expression tb_ --mode line-search
```

Searching in csv files for a regular expression and then piping the output to a file:

```ps1
file_search.exe -g data\*.csv --search-expression "\b[jJ]im\b" --mode line-regex-search --output file --file /tmp/search_line_res.txt
```

Search file using a regular expression and outputting the results to an HTML file:

```ps1
file_search.exe -g data\*.csv --search-expression "\b[jJ]im\b" --mode line-regex-search --output html --file /tmp/search_line_res.html
```