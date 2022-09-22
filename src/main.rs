extern crate glob;

use std::{fs, io};
use std::path::{Path, PathBuf};
use crate::cli::{Cli, Mode};
use clap::Parser;
use self::glob::glob;
use fs::File;
use std::process;
use crate::io_ops::read_lines;

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
    let main_file_path = &path.to_str().unwrap();
    let search_filter = search_expression_option.as_ref().unwrap();
    match read_lines(&path) {
        Ok(lines) => {
            for line in lines {
                if let Ok(s) = line {
                    if find_simple(&s, search_filter) {
                        println!("{} :: {}", main_file_path, s);
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Could not process path {:?} due to {}", path, e);
        }
    }
}

fn main() {
    let args = Cli::parse();
    let search_expression = &args.search_expression;
    let mode = &args.mode;
    match mode {
        Mode::FileName => {
            println!("Mode is file-name");
            if search_expression.is_none() {
                read_files(&args, process_path_simple);
            } else {
                read_files(&args, process_path_with_expression);
            }
        }
        Mode::Zip => {
            println!("Mode is zip");
            if search_expression.is_none() {
                eprintln!("The search expression is required when using Zip search.")
            } else {
                read_files(&args, process_zip_with_expression);
            }
        }
        Mode::LineSearch => {
            println!("Mode is line search");
            if search_expression.is_none() {
                eprintln!("Please enter the search expression with e.g: '--search-expression tb_'");
                process::exit(0x0001);
            } else {
                read_files(&args, process_line_search);
            }
        }
    }
}
