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
    match json_string_res {
        Ok(json_string) => {
            let finder = JsonPathFinder::from_str(&json_string, json_path_str).unwrap();
            let slice_of_data: Vec<&Value> = finder.find_slice();
            if !slice_of_data.is_empty() {
                output.output(format!("{} :: {:?}", path.to_str().unwrap(), slice_of_data).as_str());
            }
        }
        Err(e) => {
            output.err_output(format!("Error occurred: {:?}", e).as_str());
        }
    }
}