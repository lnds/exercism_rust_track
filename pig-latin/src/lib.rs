fn rule_1(input: &str) -> Option<String> {
    if ["a", "e", "i", "o", "u", "xr", "yt"].iter().any(|s| input.starts_with(s)){
        Some(input.to_string() + "ay")
    } else {
        None
    }
}

fn find_consonant_cluster(input: &str) -> Option<usize> {
    input.chars().enumerate().find_map(|(i, c)| match c {
        'a' | 'e' | 'i' | 'o' | 'u' => Some(i),
        _ => None,
    })
}

fn rule_2(input: &str) -> Option<String> {
    match find_consonant_cluster(input) {
        None => None,
        Some(pos) => Some(input[pos..].to_string() + &input[0..pos] + "ay"),
    }
}

fn rule_4(input: &str) -> Option<String> {
    match input.find('y') {
        None => None,
        Some(pos) if pos > 0 => Some(input[pos..].to_string() + &input[..pos] + "ay"),
        _ => None,
    }
}

fn rule_3(input: &str) -> Option<String> {
    match input.find("qu") {
        None => None,
        Some(pos) => Some(input[pos + 2..].to_string() + &input[..pos + 2] + "ay"),
    }
}

fn translate_word(input: &str) -> String {
    rule_1(input)
        .or_else(|| rule_3(input))
        .or_else(|| rule_2(input))
        .or_else(|| rule_4(input))
        .unwrap_or_else(|| input.to_string())
}

pub fn translate(input: &str) -> String {
    input
        .split_whitespace()
        .map(|s| translate_word(s))
        .collect::<Vec<String>>()
        .join(" ")
        .to_string()
}
