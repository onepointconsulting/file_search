extern crate glob;

use std::path::PathBuf;
use crate::cli::{Cli, Mode};
use clap::Parser;
use self::glob::glob;

mod cli;

fn read_files(cli: &Cli, process_fn: fn (PathBuf, &Option<String>) -> ()) {
    let glob_pattern = &cli.glob_pattern;
    let search_expression = &cli.search_expression;
    let expected = format!("Failed to read glob pattern {}", glob_pattern);
    for entry in glob(glob_pattern).expect(&expected) {
        match entry {
            Ok(path) => {
                process_fn(path, search_expression);
            }
            Err(e) => {

            }
        }
    }
}

fn process_path_simple(path: PathBuf, _: &Option<String>) {
    match path.to_str() {
        Some(s) => {
            println!("{}", s);
        }
        _ => ()
    }
}

fn process_path_with_expression(path: PathBuf, search_expression_option: &Option<String>) {
    match path.to_str() {
        Some(s) => {
            match search_expression_option {
                Some(search_filter) => {
                    if s.find(search_filter).is_some() {
                        println!("{}", s);
                    }
                }
                _ => ()
            }
        }
        _ => ()
    }
}

fn main() {
    let args = Cli::parse();
    let search_expression = &args.search_expression;
    let mode = &args.mode;
    match mode {
        Mode::FileName=> {
            println!("Mode is file-name");
            match search_expression {
                Some(se) => {
                    read_files(&args, process_path_with_expression);
                }
                None => {
                    read_files(&args, process_path_simple);
                }
            }
        }
        Mode::Zip=> {
            println!("Mode is zip. Not implemented yet");
        }
    }
}
