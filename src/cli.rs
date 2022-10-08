use clap::{Parser, ArgEnum};

#[derive(ArgEnum, Debug, Clone)]
#[clap(rename_all = "kebab_case")]
pub(crate) enum Mode {
    FileName,
    Zip,
    LineSearch,
    LineRegexSearch,
    ZipRegex,
    JsonPath
}

#[derive(ArgEnum, Debug, Clone)]
#[clap(rename_all = "kebab_case")]
pub(crate) enum Output {
    Console,
    File
}

/**
Simple binary programme used to grep files by name, or for searching inside of compressed files.
JSON path is also supported when using json-path mode. If using json path, it should be a valid json path expression.
*/
#[derive(Parser)]
pub(crate) struct Cli {
    /// The glob pattern used to list files, e.g. *.zip or /media/**/*.csv.
    #[clap(short, long)]
    pub(crate) glob_pattern: String,

    /// The search expression, like 'foo' or if using json path e.g. '$..name'. Not a regular expression unless you should use "line-search-regex"
    #[clap(short, long)]
    pub(crate) search_expression: Option<String>,

    /// The operation mode
    #[clap(short, long, arg_enum)]
    pub(crate) mode: Mode,

    /// The output mode
    #[clap(short, long, arg_enum)]
    pub(crate) output: Option<Output>,

    /// The output file in case the output parameter is "File"
    #[clap(short, long)]
    pub(crate) file: Option<String>,
}