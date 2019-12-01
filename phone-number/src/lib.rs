pub fn number(user_number: &str) -> Option<String> {
    let mut user_number = user_number.replace(|c: char| !c.is_numeric(), "");
    if user_number.len() > 10 && user_number.starts_with('1') {
        user_number = user_number[1..].to_string();
    }
    if user_number.len() != 10
        || user_number.starts_with(|c| c == '0' || c == '1')
        || user_number
            .chars()
            .nth(3)
            .map_or(false, |c| c == '0' || c == '1')
    {
        None
    } else {
        Some(user_number)
    }
}
