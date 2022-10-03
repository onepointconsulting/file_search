use std::borrow::Borrow;
use std::fs::File;
use std::io::Write;
use std::path::{Path};

pub(crate) trait OutputPrinter {
    fn output(&self, msg: &str);
    fn err_output(&self, msg: &str);
    fn get_name(&self) -> &str;
}

pub(crate) struct StdPrinter {}

impl OutputPrinter for StdPrinter {
    fn output(&self, msg: &str) {
        println!("{}", msg)
    }

    fn err_output(&self, msg: &str) {
        eprintln!("{}", msg)
    }

    fn get_name(&self) -> &str {
        return "StdPrinter"
    }
}

pub(crate) struct FilePrinter<'a> {
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

    fn output(&self, msg: &str) {
        print_msg(self.file, self.path, msg, "message");
    }

    fn err_output(&self, msg: &str) {
        print_msg(self.file, self.path, msg, "error");
    }

    fn get_name(&self) -> &str {
        return "FilePrinter"
    }
}

