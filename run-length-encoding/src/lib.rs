fn enc(current: Option<(&char, usize)>, source: &str, mut acum: String) -> String {
    if source.len() == 0 {
        if let Some((ch, len)) = current {
            if len > 1 {
                acum.push_str(&len.to_string());
            }
            acum.push(*ch);
        }
        return acum;
    }
    let ch: char = source.chars().next().unwrap();
    match current {
        None => enc(Some((&ch, 1)), &source[1..], acum),
        Some((c, len)) => {
            if ch == *c {
                enc(Some((c, len + 1)), &source[1..], acum)
            } else {
                if len > 1 {
                    acum.push_str(&len.to_string());
                }
                acum.push(*c);
                enc(Some((&ch, 1)), &source[1..], acum)
            }
        }
    }
}

pub fn encode(source: &str) -> String {
    enc(None, source, String::new())
    /*
    let mut result = String::new();
    let mut current: Option<char> = None;
    let mut count = 0;
    for c in source.chars() {
        match current {
            None => {
                current = Some(c);
                count = 1;
            }
            Some(ch) => {
                if ch == c {
                    count += 1;
                } else {
                    if count > 1 {
                        result.push_str(&count.to_string());
                    }
                    result.push(ch);
                    count = 1;
                    current = Some(c);
                }
            }
        }
    }
    if let Some(ch) = current {
        if count > 1 {
            result.push_str(&count.to_string());
        }
        result.push(ch);
    }
    result
    */
}

pub fn decode(source: &str) -> String {
    let mut result = String::new();
    let mut len = 0;
    for c in source.chars() {
        if c.is_numeric() {
            len = len * 10 + c.to_digit(10).unwrap();
        } else if len == 0 {
            result.push(c);
        } else {
            result.push_str(&c.to_string().repeat(len as usize));
            len = 0;
        }
    }
    result
}
