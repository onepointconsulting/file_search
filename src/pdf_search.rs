extern crate lopdf;

use std::path::PathBuf;
use pdf_extract::extract_text;
use crate::OutputPrinter;
use crate::finders::find_simple_pos;
use unicode_segmentation::UnicodeSegmentation;

pub(crate) fn process_pdf_simple_search(path: PathBuf, search_expression_option: &Option<String>,
                                        output: &mut dyn OutputPrinter) {
    let extracted = extract_text(path.clone());
    let file_name = path.to_str().expect("Could not extract file name from path");
    match extracted {
        Ok(content) => {
            let search_expression = search_expression_option.as_ref().unwrap();
            let content_str = content.as_str();
            let found = find_simple_pos(content_str, search_expression);
            if found.is_some() {
                let example_vec = UnicodeSegmentation::grapheme_indices(content_str, true)
                    .collect::<Vec<(usize, &str)>>();
                let example = &example_vec[..found.unwrap() + search_expression.len()].iter()
                    .map(|x| x.1).collect::<String>();
                output.output_with_stats(format!("{} :: {} :: @@{}@@", file_name, found.unwrap(), example)
                    .as_str());
            }
        }
        Err(e) => {
            output.err_output(format!("Could not extract text from '{}': {:?}", file_name, e).as_str());
        }
    }
}