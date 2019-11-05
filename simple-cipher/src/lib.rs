use rand::prelude::*;

fn transform(key: &str, s: &str, f: fn(u8, u8) -> Option<char>) -> Option<String> {
    if key.is_empty() {
        None
    } else {
        s.bytes()
            .zip(key.bytes().cycle())
            .map(|(c, k)| f(c, k))
            .collect()
    }
}

pub fn encode(key: &str, s: &str) -> Option<String> {
    transform(key, s, encode_char)
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    transform(key, s, decode_char)
}

fn encode_char(c: u8, key: u8) -> Option<char> {
    match key {
        b'a'..=b'z' => Some(char::from(b'a' + (26 + (c - b'a') + (key - b'a')) % 26)),
        _ => None,
    }
}

fn decode_char(c: u8, key: u8) -> Option<char> {
    match key {
        b'a'..=b'z' => Some(char::from(b'a' + (26 + (c - b'a') - (key - b'a')) % 26)),
        _ => None,
    }
}

pub fn encode_random(s: &str) -> (String, String) {
    let mut rnd = thread_rng();
    let key: String = (0..100)
        .map(|_| (rnd.gen_range(b'a', b'z' + 1)) as char)
        .collect();
    let s = encode(&key, s).unwrap();
    (key, s)
}
