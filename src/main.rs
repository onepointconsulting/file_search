extern crate glob;
extern crate core;

use fs::File;
use std::{fs};
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::path::{Path, PathBuf};
use std::process;

use clap::Parser;
use colored::Colorize;
use fancy_regex::Regex;

use crate::cli::{Cli, Mode, Output};
use crate::finders::find_simple;
use crate::html_printer::HtmlPrinter;
use crate::io_ops::{LINE_ENDING, read_lines};
use crate::json_path_search::process_file_with_json_path;
use crate::pdf_search::process_pdf_simple_search;
use crate::result_printer::{FilePrinter, OutputPrinter, Statistics, StdPrinter};

use self::glob::glob;

mod cli;
mod io_ops;
mod result_printer;
mod json_path_search;
mod pdf_search;
mod finders;
mod html_printer;

fn read_files(cli: &Cli, process_fn: fn(PathBuf, &Option<String>, output: &mut dyn OutputPrinter) -> (), output: &mut dyn OutputPrinter) {
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

fn process_path_simple(path: PathBuf, _: &Option<String>, output: &mut dyn OutputPrinter) {
    match path.to_str() {
        Some(s) => {
            output.output_with_stats(s);
        }
        None => {
            output.err_output("Nothing to print")
        }
    }
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
                                output: &mut dyn OutputPrinter) {
    match path.to_str() {
        Some(s) => {
            match search_expression_option {
                Some(search_filter) => {
                    if find_simple(s, search_filter) {
                        output.output_with_stats(s);
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

fn process_zip_with_expression(path: PathBuf, search_expression_option: &Option<String>, output: &mut dyn OutputPrinter) {
    process_zip_with_expression_generic(path, search_expression_option, find_simple, output);
}

fn process_zip_with_regex(path: PathBuf, search_expression_option: &Option<String>, output: &mut dyn OutputPrinter) {
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
                                          output: &mut dyn OutputPrinter) {
    let main_file_path = &path.to_str().unwrap();
    let zip_file = File::open(&path).unwrap();
    let zip_result = zip::ZipArchive::new(&zip_file);
    match zip_result {
        Ok(mut archive) => {
            let len = &archive.len();
            let search_filter = search_expression_option.as_ref().unwrap();
            for i in 0..*len {
                let file = archive.by_index(i).unwrap();
                let file_name = file.name();
                if find_func(file_name, search_filter) {
                    output.output_with_stats(format!("{} :: {}", main_file_path, file_name).as_str());
                }
            }
        }
        Err(e) => {
            output.err_output(format!("{:?}", e).as_str())
        }
    }
}

fn process_line_search(path: PathBuf, search_expression_option: &Option<String>, output: &mut dyn OutputPrinter) {
    process_line_search_generic(path, search_expression_option, find_simple, output);
}

fn process_regex_search(path: PathBuf, search_expression_option: &Option<String>, output: &mut dyn OutputPrinter) {
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
                                  output: &mut dyn OutputPrinter) {
    let main_file_path = &path.to_str().unwrap();
    let search_filter = search_expression_option.as_ref().unwrap();
    match read_lines(&path) {
        Ok(lines) => {
            for (linenumber, line) in lines.enumerate() {
                if let Ok(s) = line {
                    if search_fn(&s, search_filter) {
                        let mut content = format!("{}", main_file_path);
                        if output.get_name().eq("StdPrinter") {
                            content = content.bold().parse().unwrap();
                        }
                        output.output_with_stats(format!("{} :: {} :: {}", content, linenumber, s.trim()).as_str());
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
    process_fn: fn(PathBuf, &Option<String>, output: &mut dyn OutputPrinter),
    output: &mut dyn OutputPrinter,
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
    let statistics = Statistics { hits: 0, errors: 0 };
    let mut printer: &mut dyn OutputPrinter = &mut StdPrinter { statistics };
    let mut file_printer_obj;
    let mut html_printer_obj;
    let file;
    let mut std_printer = StdPrinter { statistics };

    match output_option {
        Some(output) => {
            match output {
                Output::Console => {
                    printer = &mut std_printer
                }
                Output::File => {
                    match file_option {
                        Some(f) => {
                            let file_path = create_file(f);
                            let written_file_result = prepare_file(&file_path);
                            file = written_file_result.unwrap();
                            file_printer_obj = FilePrinter {
                                statistics,
                                path: &file_path,
                                file: &file,
                            };
                            printer = &mut file_printer_obj;
                        }
                        None => {
                            // The user probably forgot about the file
                            printer = &mut std_printer
                        }
                    }
                }
                Output::HTML => {
                    match file_option {
                        Some(f) => {
                            let file_path = create_file(f);
                            let written_file_result = prepare_file(&file_path);
                            file = written_file_result.unwrap();
                            html_printer_obj = HtmlPrinter {
                                statistics,
                                path: &file_path,
                                file: &file,
                            };
                            printer = &mut html_printer_obj;
                        }
                        None => {
                            // The user probably forgot about the file
                            printer = &mut std_printer
                        }
                    }
                }
            }
        }
        None => {}
    }

    print_cmd_options(&args, printer);
    process_all_modes(&args, search_expression, mode, printer);
    printer.print_stats();
}

fn prepare_file(file_path: &&Path) -> std::io::Result<File> {
    OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(&file_path)
}

fn create_file(f: &String) -> &Path {
    let file_path = Path::new(f);
    if file_path.exists() {
        fs::remove_file(file_path).expect(
            format!("Could not remove {}.", file_path.display().to_string()).as_str());
    }
    file_path
}

fn print_cmd_options(args: &Cli, printer: &mut dyn OutputPrinter) {
    let mut print_map = HashMap::new();
    print_map.insert("Mode".to_string(), format!("{:?}", args.mode));
    print_map.insert("Glob".to_string(), format!("{:?}", args.glob_pattern));
    if args.search_expression.is_some() {
        print_map.insert("Search".to_string(), format!("{:?}", args.search_expression.clone().unwrap()));
    }
    if args.file.is_some() {
        print_map.insert("File".to_string(), format!("{:?}", args.file.clone().unwrap()));
    }
    printer.print_param_map(print_map);
}

fn process_all_modes(args: &Cli,
                     search_expression: &Option<String>,
                     mode: &Mode, printer: &mut dyn OutputPrinter) {
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
        Mode::JsonPath => {
            execute_on_expression(&args,
                                  handle_missing_search_expression,
                                  process_file_with_json_path, printer);
        }
        Mode::PdfSearch => {
            execute_on_expression(&args,
                                  handle_missing_search_expression,
                                  process_pdf_simple_search, printer);
        }
    }
}
