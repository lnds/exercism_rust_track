pub fn encode(source: &str) -> String {
    let r = source.chars().fold(None, |acum, c| match acum {
        None => Some((c, 1, String::new())),
        Some((ch, len, s)) => {
            if c == ch {
                Some((ch, len + 1, s))
            } else if len > 1 {
                Some((c, 1, s + &len.to_string() + &ch.to_string()))
            } else {
                Some((c, 1, s + &ch.to_string()))
            }
        }
    });
    match r {
        None => String::new(),
        Some((ch, 1, result)) => result + &(ch.to_string()),
        Some((ch, len, result)) => result + &len.to_string() + &(*ch.to_string()),
    }
}

pub fn decode(source: &str) -> String {
    source
        .chars()
        .fold((String::new(), 0), |(result, len), c| {
            if c.is_numeric() {
                (result, len * 10 + c.to_digit(10).unwrap())
            } else if len == 0 {
                (result + &c.to_string(), 0)
            } else {
                (result + &c.to_string().repeat(len as usize), 0)
            }
        })
        .0
}
