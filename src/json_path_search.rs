use std::fs;
use std::path::PathBuf;
use jsonpath_rust::JsonPathFinder;

use serde_json::Value;

use crate::OutputPrinter;

pub(crate) fn process_file_with_json_path(path: PathBuf, search_expression_option: &Option<String>,
                                          output: &dyn OutputPrinter) {
    let path_clone = path.clone();
    let json_path_str = search_expression_option.as_ref().unwrap();
    let json_string_res = fs::read_to_string(path_clone);
    let json_string = json_string_res.unwrap();
    let finder = JsonPathFinder::from_str(&json_string, json_path_str).unwrap();
    let slice_of_data: Vec<&Value> = finder.find_slice();
    output.output(format!("{} :: {:?}", path.to_str().unwrap(), slice_of_data).as_str());
}