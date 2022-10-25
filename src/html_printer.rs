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

    fn start_row() -> String {
        "<tr>".to_string()
    }
}

macro_rules! td_format {() => ("<tr><td>{}</td><td>{}</td></tr>")}

macro_rules! simple_td_format {() => ("<td>{}</td>")}

macro_rules! common_header {() => ("<thead>
            <tr>
                <th>Parameter</th>
                <th>Value</th>
              </tr>
        </thead>")}

impl OutputPrinter for HtmlPrinter<'_> {
    fn print_param_map(&mut self, map: HashMap<String, String>) {
        let mut table_content = "".to_string();
        for (key, value) in &map {
            let string = format!(td_format!(), key, value);
            table_content += &string;
        }
        let msg = format!(r###"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Document</title>
    <style>
        body {{
            font-family: Arial, Helvetica, sans-serif
        }}
        table, th, td {{
            border: 1px solid white;
            border-collapse: collapse;
        }}
        th, td {{
            background-color: #efefef;
        }}
        th, td {{
            padding: 3px 10px
        }}
    </style>
</head>
<body>
    <h1>File Search</h1>
    <h2>Parameters</h2>
    <table>
        {}
        <tbody>
            {}
        </tbody>
    </table>
    <h2>Results</h2>
    <table>
        <thead>
            <tr>
                <th>File name</th>
                <th>Position</th>
                <th></th>
            <tr>
        </thead>
        <tbody>
"###, common_header!(), table_content);

        self.print_to_file(msg.as_str(), "message")
    }

    fn output_with_stats(&mut self, msg: &str) {
        self.output(msg);
        self.statistics.increase_hits();
    }

    fn output(&mut self, msg: &str) {
        let splits = msg.split("::");
        let mut acc = Self::start_row();
        for s in splits {
            acc += format!(simple_td_format!(), s).as_str()
        }
        acc += "</tr>";
        self.print_to_file(acc.as_str(), "message");
    }

    fn err_output(&mut self, msg: &str) {
        let mut acc = Self::start_row();
        acc += format!(simple_td_format!(), msg).as_str();
        acc += "</tr>";
        self.statistics.increase_errors();
    }

    fn get_name(&self) -> &str {
        "HtmlPrinter"
    }

    fn print_stats(&mut self) {
        let hits_td = format!(td_format!(), "Hits", self.statistics.hits);
        let error_td = format!(td_format!(), "Errors", self.statistics.errors);
        self.print_to_file(format!(r###"
            </tbody>
        </table>
        <h2>Stats</h2>
        <table>
            {}
            <tbody>
                {}
                {}
            </tbody>
        </table>
    </body>
</html>
"###, common_header!(), hits_td, error_td).as_str(), "message")
    }
}