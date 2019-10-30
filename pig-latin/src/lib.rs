const VOWELS: [&str; 7] = ["a", "e", "i", "o", "u", "xr", "yr"];
const CONSONANTS: [&str; 25] = [
    "b", "ch", "c", "d", "f", "g", "h", "j", "k", "l", "m", "n", "p", "q", "qu", "r", "sch", "s",
    "thr", "th", "t", "v", "w", "x", "z",
];
const Y: [&str; 5] = ["ya", "ye", "yi", "yo", "yu"];

fn translate_word(input: &str) -> String {
    let mut result = input.to_string();
    let m = input.to_lowercase();
    if VOWELS.iter().any(|s| m.starts_with(s)) {
        result.push_str("ay");
        return result;
    }
    if let Some(pos) = m.find("y") {
        if pos != input.len() - 1 {
            if pos > 0 {
                let sufix = result.get(0..pos).unwrap().to_string() + "ay";
                result = result.get(pos..).unwrap().to_string() + &sufix;
            } else {
                if Y.iter().any(|s| m.starts_with(s)) {
                    result = result.get(1..).unwrap().to_string() + "yay"
                } else {
                    result = result + "ay";
                }
            }
            return result;
        }
    }
    if let Some(prefix) = CONSONANTS.iter().find(|&s| m.starts_with(s)) {
        let mut pos = prefix.len();
        let inc = if *prefix == "q" { 1 } else { 2 };
        if let Some(s) = result.get(0..1 + inc) {
            if s.ends_with("qu") {
                pos += inc;
            }
        };
        let sufix = result.get(0..pos).unwrap().to_string() + "ay";
        result = result.get(pos..).unwrap().to_string() + &sufix;
    }
    result
}

pub fn translate(input: &str) -> String {
    input
        .split_whitespace()
        .map(|s| translate_word(s))
        .collect::<Vec<String>>()
        .join(" ").to_string()
}
