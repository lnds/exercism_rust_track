pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(|c: char| c.is_whitespace() || c == '-')
        .flat_map(|w| {
            let r = w.chars().skip_while(|c| c.is_uppercase()).filter(|c| c.is_uppercase());
            w.chars().take(1).chain(r)
        }).flat_map(char::to_uppercase).collect()
}
