

pub(crate) fn find_simple(content: &str, search_filter: &String) -> bool {
    content.find(search_filter).is_some()
}

pub(crate) fn find_simple_pos(content: &str, search_filter: &String) -> Option<usize> {
    content.find(search_filter)
}