extern crate glob;
extern crate core;

use fs::File;
use std::{fs};
use std::fs::OpenOptions;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::process;

use clap::Parser;
use colored::Colorize;
use fancy_regex::Regex;

use crate::cli::{Cli, Mode, Output};
use crate::io_ops::read_lines;
use crate::result_printer::{FilePrinter, OutputPrinter, StdPrinter};

use self::glob::glob;

mod cli;
mod io_ops;
mod result_printer;

fn read_files(cli: &Cli, process_fn: fn(PathBuf, &Option<String>, output: &dyn OutputPrinter) -> (), output: &dyn OutputPrinter) {
    let glob_pattern = &cli.glob_pattern;
    let search_expression = &cli.search_expression;
    let expected = format!("Failed to read glob pattern {}", glob_pattern);
    for entry in glob(glob_pattern).expect(&expected) {
        match entry {
            Ok(path) => {
                process_fn(path, search_expression, output);
            }
            Err(_) => {}
        }
    }
}

fn process_path_simple(path: PathBuf, _: &Option<String>, output: &dyn OutputPrinter) {
    match path.to_str() {
        Some(s) => {
            output.output(s);
        }
        None => {
            output.err_output("Nothing to print")
        }
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
            Some(_) => {
                // println!("Start position: {}", m.start());
                return true;
            }
            None => {}
        }
    }
    return false;
}

fn process_path_with_expression(path: PathBuf, search_expression_option: &Option<String>,
                                output: &dyn OutputPrinter) {
    match path.to_str() {
        Some(s) => {
            match search_expression_option {
                Some(search_filter) => {
                    if find_simple(s, search_filter) {
                        output.output(s);
                    }
                }
                _ => ()
            }
        }
        None => {
            output.err_output("Path not found")
        }
    }
}

fn process_zip_with_expression(path: PathBuf, search_expression_option: &Option<String>, output: &dyn OutputPrinter) {
    process_zip_with_expression_generic(path, search_expression_option, find_simple, output);
}

fn process_zip_with_regex(path: PathBuf, search_expression_option: &Option<String>, output: &dyn OutputPrinter) {
    match search_expression_option {
        Some(search_expression) => {
            let re = Regex::new(search_expression).expect("Invalid regex");
            process_zip_with_expression_generic(path, &Some(re), find_regex,
                                                output);
        }
        None => {}
    }
}

fn process_zip_with_expression_generic<T>(path: PathBuf, search_expression_option: &Option<T>,
                                          find_func: fn(content: &str, search_filter: &T) -> bool,
                                          output: &dyn OutputPrinter) {
    let main_file_path = &path.to_str().unwrap();
    let zip_file = File::open(&path).unwrap();
    let mut archive = zip::ZipArchive::new(&zip_file).unwrap();
    let len = &archive.len();
    let search_filter = search_expression_option.as_ref().unwrap();
    for i in 0..*len {
        let file = archive.by_index(i).unwrap();
        let file_name = file.name();
        if find_func(file_name, search_filter) {
            output.output(format!("{} :: {}", main_file_path, file_name).as_str());
        }
    }
}

fn process_line_search(path: PathBuf, search_expression_option: &Option<String>, output: &dyn OutputPrinter) {
    process_line_search_generic(path, search_expression_option, find_simple, output);
}

fn process_regex_search(path: PathBuf, search_expression_option: &Option<String>, output: &dyn OutputPrinter) {
    match search_expression_option {
        Some(search_expression) => {
            let re = Regex::new(search_expression).expect("Invalid regex");
            process_line_search_generic(path, &Some(re), find_regex, output);
        }
        None => {}
    }
}

fn process_line_search_generic<T>(path: PathBuf, search_expression_option: &Option<T>,
                                  search_fn: fn(&str, &T) -> bool,
                                  output: &dyn OutputPrinter) {
    let main_file_path = &path.to_str().unwrap();
    let search_filter = search_expression_option.as_ref().unwrap();
    match read_lines(&path) {
        Ok(lines) => {
            for (linenumber, line) in lines.enumerate() {
                if let Ok(s) = line {
                    if search_fn(&s, search_filter) {
                        output.output(format!("{} :: {} :: {}", format!("{}", main_file_path).bold(), linenumber, s.trim()).as_str());
                    }
                }
            }
        }
        Err(e) => {
            output.err_output(format!("Could not process path {:?} due to {}", path, e).as_str());
        }
    }
}

fn handle_missing_search_expression() {
    eprintln!("Please enter the search expression with e.g: '--search-expression tb_'");
    process::exit(0x0001);
}

fn execute_on_expression(
    args: &Cli, missing_func: fn() -> (),
    process_fn: fn(PathBuf, &Option<String>, output: &dyn OutputPrinter),
    output: &dyn OutputPrinter,
) {
    let search_expression = &args.search_expression;
    if search_expression.is_none() {
        missing_func()
    } else {
        read_files(&args, process_fn, output);
    }
}

fn main() {
    let args = Cli::parse();
    let search_expression = &args.search_expression;
    let mode = &args.mode;
    let output_option: &Option<Output> = &args.output;
    let file_option: &Option<String> = &args.file;
    let mut printer: &dyn OutputPrinter = &StdPrinter {};
    let file;
    let file_printer;

    match output_option {
        Some(output) => {
            match output {
                Output::Console => {
                    printer = &StdPrinter {}
                }
                Output::File => {
                    match file_option {
                        Some(f) => {
                            let file_path = Path::new(f);
                            let written_file_result = OpenOptions::new()
                                .write(true)
                                .create(true)
                                .append(true)
                                .open(&file_path);
                            file = written_file_result.unwrap();
                            file_printer = FilePrinter {
                                path: &file_path,
                                file: &file,
                            };
                            printer = &file_printer
                        }
                        None => {
                            printer = &StdPrinter {}
                        }
                    }
                }
            }
        }
        None => {}
    }

    printer.output(format!("Mode is {}", format!("{:?}", mode)).as_str());

    process_all_modes(&args, search_expression, mode, printer)
}

fn process_all_modes(args: &Cli,
                     search_expression: &Option<String>,
                     mode: &Mode, mut printer: &dyn OutputPrinter) {
    match mode {
        Mode::FileName => {
            read_files(&args,
                       if search_expression.is_none() { process_path_simple } else { process_path_with_expression }, printer);
        }
        Mode::Zip => {
            execute_on_expression(&args,
                                  handle_missing_search_expression,
                                  process_zip_with_expression, printer);
        }
        Mode::LineSearch => {
            execute_on_expression(&args,
                                  handle_missing_search_expression,
                                  process_line_search, printer);
        }
        Mode::LineRegexSearch => {
            execute_on_expression(&args,
                                  handle_missing_search_expression,
                                  process_regex_search, printer);
        }
        Mode::ZipRegex => {
            execute_on_expression(&args,
                                  handle_missing_search_expression,
                                  process_zip_with_regex, printer);
        }
    }
}
