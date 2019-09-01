pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    for w in digits.chars().collect::<Vec<char>>().windows(len) {
        result.push(w.iter().collect::<String>());
    }
    result
}
