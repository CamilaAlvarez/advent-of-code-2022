pub fn get_item_value(item: char) -> u32 {
    if item.is_uppercase() {
        get_upper_case_value(item)
    } else {
        get_lowercase_value(item)
    }
}

fn get_lowercase_value(item: char) -> u32 {
    item as u32 - 'a' as u32 + 1
}

fn get_upper_case_value(item: char) -> u32 {
    item as u32 - 'A' as u32 + 27
}
