const VOWELS :[&str;7] = ["a","e","i","o","u","xr", "yr"];

pub fn translate(input: &str) -> String {
    let mut result = input.to_string();
    let m = input.to_lowercase();
    if VOWELS.iter().any(|s| m.starts_with(s)) {
        result.push_str("ay")
    }
    result
}
