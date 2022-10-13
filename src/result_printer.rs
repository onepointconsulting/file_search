use std::borrow::Borrow;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::{Path};
use crate::LINE_ENDING;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Statistics {
    pub hits: u32,
    pub errors: u32
}

pub(crate) trait OutputPrinter {
    fn print_param_map(&mut self, map: HashMap<String, String>);
    fn output_with_stats(&mut self, msg: &str);
    fn output(&mut self, msg: &str);
    fn err_output(&mut self, msg: &str);
    fn get_name(&self) -> &str;
    fn print_stats(&mut self);
}

pub(crate) struct StdPrinter {
    pub(crate) statistics: Statistics
}

impl Statistics {
    fn increase_hits(&mut self) {
        self.hits += 1;
    }
    fn increase_errors(&mut self) {
        self.hits += 1;
    }
}

macro_rules! kv_format {() => ("{:8} -> {}")}

impl OutputPrinter for StdPrinter {
    fn print_param_map(&mut self, map: HashMap<String, String>) {
        for (key, value) in &map {
            self.output(format!(kv_format!(), key, value).as_str());
        }
    }

    fn output_with_stats(&mut self, msg: &str) {
        println!("{}", msg);
        self.statistics.increase_hits();
    }

    fn output(&mut self, msg: &str) {
        println!("{}", msg);
    }

    fn err_output(&mut self, msg: &str) {
        eprintln!("{}", msg);
        self.statistics.increase_errors();
    }

    fn get_name(&self) -> &str {
        "StdPrinter"
    }

    fn print_stats(&mut self) {
        println!("Hits:   {}", self.statistics.hits);
        println!("Errors: {}", self.statistics.errors);
    }
}

pub(crate) struct FilePrinter<'a> {
    pub(crate) statistics: Statistics,
    pub(crate) path: &'a Path,
    pub(crate) file: &'a File
}

pub(crate) fn print_msg(file: &File, path: &Path, msg: &str, what: &str) {

    match file.borrow().write_all(format!("{}\r\n", msg).as_bytes()) {
        Err(why) => panic!("Could not write {} to {}: {}", what, path.display(), why),
        Ok(_) => {}
    }
}

impl FilePrinter<'_> {
    fn print_to_file(&mut self, msg: &str, what: &str) {
        print_msg(self.file, self.path, msg, what);
    }
}

impl OutputPrinter for FilePrinter<'_> {

    fn print_param_map(&mut self, map: HashMap<String, String>) {
        for (key, value) in &map {
            self.output(format!(kv_format!(), key, value).as_str());
        }
        self.output(LINE_ENDING);
    }

    fn output_with_stats(&mut self, msg: &str) {
        self.print_to_file(msg, "message");
        self.statistics.increase_hits();
    }

    fn output(&mut self, msg: &str) {
        self.print_to_file(msg, "message");
    }

    fn err_output(&mut self, msg: &str) {
        self.print_to_file(msg, "error");
        self.statistics.increase_errors();
    }

    fn get_name(&self) -> &str {
        "FilePrinter"
    }

    fn print_stats(&mut self) {
        let messages = [
            format!("Hits:   {}", self.statistics.hits),
            format!("Errors: {}", self.statistics.errors)
        ];
        for msg in messages {
            self.print_to_file(&msg, "message");
            print_msg(self.file, self.path, &msg, "message");
        }
    }
}

