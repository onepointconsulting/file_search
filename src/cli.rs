use clap::{Parser, ArgEnum};
use std::str::FromStr;
use std::fmt;

#[derive(ArgEnum, Debug, Clone)]
#[clap(rename_all = "kebab_case")]
enum Mode {
    FileName,
    Zip
}

/// Simple binary programme used to grep files by name, or for searching inside of compressed files.
#[derive(Parser)]
pub(crate) struct Cli {
    /// The glob pattern used to list files, e.g. *.zip or /media/**/*.csv.
    #[clap(short, long)]
    pub(crate) glob_pattern: String,

    /// The search expression, like 'foo'. Not a regular expression.
    #[clap(short, long)]
    pub(crate) search_expression: Option<String>,

    /// The operation mode
    #[clap(short, long, arg_enum)]
    pub(crate) mode: Mode
}