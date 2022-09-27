
pub(crate) trait OutputPrinter {
    fn output(&self, msg: &str);
    fn err_output(&self, msg: &str);
}

pub(crate) struct StdPrinter {

}

impl OutputPrinter for StdPrinter {
    fn output(&self, msg: &str) {
        println!("{}", msg)
    }

    fn err_output(&self, msg: &str) {
        eprintln!("{}", msg)
    }
}