fn is_all_uppercase(phrase: &str) -> bool {
    !phrase.is_empty()
        && phrase
            .chars()
            .filter(|c| c.is_alphabetic())
            .all(|c| c.is_uppercase())
}

fn is_camel_case(phrase: &str) -> bool {
    let count_uc = phrase.chars().filter(|c| c.is_uppercase()).count();
    !is_all_uppercase(phrase) && count_uc < phrase.len() && count_uc > 1
}

pub fn abbreviate(phrase: &str) -> String {
    let words: Vec<&str> = phrase
        .split(|c: char| c.is_whitespace() || c == '-')
        .collect();
    let mut s = String::new();
    for w in words {
        if is_camel_case(w) {
            s.push_str(w.chars().filter(|c| c.is_uppercase()).collect::<String>().as_str());
        } else {
            s.push(w.chars().next().unwrap().to_uppercase().next().unwrap())
        }
    }
    s
}
