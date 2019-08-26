fn ask(message: &str) -> bool {
    message.ends_with('?')
}

fn yell(message: &str) -> bool {
    let a = message.chars().filter(|c|c.is_alphabetic()).count();
    let b = message.chars().filter(|c|c.is_uppercase()).count();
    a > 0 && b > 0 && a == b
}


pub fn reply(message: &str) -> &str {
    match message.trim() {
        msg if ask(msg) && yell(msg) => "Calm down, I know what I\'m doing!",
        msg if ask(msg) => "Sure.",
        msg if yell(msg) => "Whoa, chill out!",
        msg if msg.is_empty() => "Fine. Be that way!",
        _ => "Whatever."
    }
}
