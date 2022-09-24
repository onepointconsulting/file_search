extern crate glob;

use fs::File;
use std::{fs};
use std::path::{PathBuf};
use std::process;

use clap::Parser;
use colored::Colorize;
use fancy_regex::Regex;

use crate::cli::{Cli, Mode};
use crate::io_ops::read_lines;

use self::glob::glob;

mod cli;
mod io_ops;

fn read_files(cli: &Cli, process_fn: fn(PathBuf, &Option<String>) -> ()) {
    let glob_pattern = &cli.glob_pattern;
    let search_expression = &cli.search_expression;
    let expected = format!("Failed to read glob pattern {}", glob_pattern);
    for entry in glob(glob_pattern).expect(&expected) {
        match entry {
            Ok(path) => {
                process_fn(path, search_expression);
            }
            Err(_) => {}
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

fn find_simple(content: &str, search_filter: &String) -> bool {
    content.find(search_filter).is_some()
}

fn find_regex(content: &str, search_filter: &Regex) -> bool {
    let result = search_filter.find(content);
    if result.is_ok() {
        let match_option = result.unwrap();
        match match_option {
            Some(m) => {
                // println!("Start position: {}", m.start());
                return true;
            }
            None => {}
        }
    }
    return false;
}

fn process_path_with_expression(path: PathBuf, search_expression_option: &Option<String>) {
    match path.to_str() {
        Some(s) => {
            match search_expression_option {
                Some(search_filter) => {
                    if find_simple(s, search_filter) {
                        println!("{}", s);
                    }
                }
                _ => ()
            }
        }
        None => {
            eprintln!("Path {:?} not found", path)
        }
    }
}

fn process_zip_with_expression(path: PathBuf, search_expression_option: &Option<String>) {
    let main_file_path = &path.to_str().unwrap();
    let zip_file = File::open(&path).unwrap();
    let mut archive = zip::ZipArchive::new(&zip_file).unwrap();
    let len = &archive.len();
    let search_filter = search_expression_option.as_ref().unwrap();
    for i in 0..*len {
        let file = archive.by_index(i).unwrap();
        let file_name = file.name();
        if find_simple(file_name, search_filter) {
            println!("{} :: {}", main_file_path, file_name);
        }
    }
}

fn process_line_search(path: PathBuf, search_expression_option: &Option<String>) {
    process_line_search_generic(path, search_expression_option, find_simple);
}

fn process_regex_search(path: PathBuf, search_expression_option: &Option<String>) {
    match search_expression_option {
        Some(search_expression) => {
            let re = Regex::new(search_expression).expect("Invalid regex");
            let regex_option = Some(re);
            process_line_search_generic(path, &regex_option, find_regex);
        }
        None => {}
    }
}

fn process_line_search_generic<T>(path: PathBuf, search_expression_option: &Option<T>, search_fn: fn(&str, &T) -> bool) {
    let main_file_path = &path.to_str().unwrap();
    let search_filter = search_expression_option.as_ref().unwrap();
    match read_lines(&path) {
        Ok(lines) => {
            for (linenumber, line) in lines.enumerate() {
                if let Ok(s) = line {
                    if search_fn(&s, search_filter) {
                        println!("{} :: {} :: {}", main_file_path, linenumber, s.trim());
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Could not process path {:?} due to {}", path, e);
        }
    }
}

fn handle_missing_search_expression() {
    eprintln!("Please enter the search expression with e.g: '--search-expression tb_'");
    process::exit(0x0001);
}

fn main() {
    let args = Cli::parse();
    let search_expression = &args.search_expression;
    let mode = &args.mode;
    println!("Mode is {}", format!("{:?}", mode).bold());
    match mode {
        Mode::FileName => {
            if search_expression.is_none() {
                read_files(&args, process_path_simple);
            } else {
                read_files(&args, process_path_with_expression);
            }
        }
        Mode::Zip => {
            if search_expression.is_none() {
                eprintln!("The search expression is required when using Zip search.")
            } else {
                read_files(&args, process_zip_with_expression);
            }
        }
        Mode::LineSearch => {
            if search_expression.is_none() {
                handle_missing_search_expression();
            } else {
                read_files(&args, process_line_search);
            }
        }
        Mode::LineRegexSearch => {
            if search_expression.is_none() {
                handle_missing_search_expression();
            } else {
                read_files(&args, process_regex_search);
            }
        }
    }
}
