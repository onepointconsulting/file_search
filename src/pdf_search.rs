extern crate lopdf;

use std::path::PathBuf;
use pdf_extract::extract_text;
use crate::OutputPrinter;
use crate::finders::find_simple_pos;

pub(crate) fn process_pdf_simple_search(path: PathBuf, search_expression_option: &Option<String>,
                                        output: &mut dyn OutputPrinter) {
    let extracted = extract_text(path.clone());
    let file_name = path.to_str().expect("Could not extract file name from path");
    match extracted {
        Ok(content) => {
            let json_path_str = search_expression_option.as_ref().unwrap();
            let found = find_simple_pos(content.as_str(), json_path_str);
            if found.is_some() {
                output.output_with_stats(format!("{} :: {}", file_name, found.unwrap()).as_str());
            }
        }
        Err(e) => {
            output.err_output(format!("Could not extract text from '{}': {:?}", file_name, e).as_str());
        }
    }
}