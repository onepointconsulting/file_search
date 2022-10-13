use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use crate::{OutputPrinter, Statistics};
use crate::result_printer::print_msg;

pub(crate) struct HtmlPrinter<'a> {
    pub(crate) statistics: Statistics,
    pub(crate) path: &'a Path,
    pub(crate) file: &'a File,
}

impl HtmlPrinter<'_> {
    fn print_to_file(&mut self, msg: &str, what: &str) {
        print_msg(self.file, self.path, msg, what);
    }
}

impl OutputPrinter for HtmlPrinter<'_> {
    fn print_param_map(&mut self, map: HashMap<String, String>) {
        let msg = format!(r###"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Document</title>
</head>
<body>
    <h1>File Search</h1>
    <table>
        <thead>
            <tr>
                <th>Parameter</th>
                <th>Value</th>
              </tr>
        </thead>
    </table>
"###);
        self.print_to_file(msg.as_str(), "message")
    }

    fn output_with_stats(&mut self, msg: &str) {}

    fn output(&mut self, msg: &str) {}

    fn err_output(&mut self, msg: &str) {}

    fn get_name(&self) -> &str {
        "OutputPrinter"
    }

    fn print_stats(&mut self) {
        self.print_to_file(format!(r###"
</body>
</html>
"###).as_str(), "message")
    }
}