use std::borrow::Borrow;
use std::fs::File;
use std::io::Write;
use std::path::{Path};

pub(crate) struct Statistics {
    pub hits: u32,
    pub errors: u32
}

pub(crate) trait OutputPrinter {
    fn output_with_stats(&mut self, msg: &str);
    fn output(&mut self, msg: &str);
    fn err_output(&mut self, msg: &str);
    fn get_name(&self) -> &str;
    fn print_stats(&self);
}

pub(crate) struct StdPrinter {
    pub statistics: Statistics
}

impl Statistics {
    fn increase_hits(&mut self) {
        self.hits += 1;
    }
    fn increase_errors(&mut self) {
        self.hits += 1;
    }
}

impl OutputPrinter for StdPrinter {
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
        return "StdPrinter"
    }

    fn print_stats(&self) {
        println!("Hits:   {}", self.statistics.hits);
        println!("Errors: {}", self.statistics.errors);
    }
}

pub(crate) struct FilePrinter<'a> {
    pub(crate) statistics: Statistics,
    pub(crate) path: &'a Path,
    pub(crate) file: &'a File
}

fn print_msg(file: &File, path: &Path, msg: &str, what: &str) {

    match file.borrow().write_all(format!("{}\r\n", msg).as_bytes()) {
        Err(why) => panic!("Could not write {} to {}: {}", what, path.display(), why),
        Ok(_) => {}
    }
}

impl OutputPrinter for FilePrinter<'_> {

    fn output_with_stats(&mut self, msg: &str) {
        print_msg(self.file, self.path, msg, "message");
        self.statistics.increase_hits();
    }

    fn output(&mut self, msg: &str) {
        print_msg(self.file, self.path, msg, "message");
    }

    fn err_output(&mut self, msg: &str) {
        print_msg(self.file, self.path, msg, "error");
        self.statistics.increase_errors();
    }

    fn get_name(&self) -> &str {
        return "FilePrinter"
    }

    fn print_stats(&self) {
        let messages = [
            format!("Hits:   {}", self.statistics.hits),
            format!("Errors: {}", self.statistics.errors)
        ];
        for msg in messages {
            print_msg(self.file, self.path, &msg, "message");
        }
    }
}

