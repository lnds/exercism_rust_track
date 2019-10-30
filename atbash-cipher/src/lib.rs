use std::collections::HashMap;

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    let alphabet = (b'a'..=b'z').map(char::from);
    let tebahpla = (b'a'..=b'z').rev().map(char::from);
    let collate = alphabet.zip(tebahpla).collect::<HashMap<_, _>>();
    plain
        .to_lowercase()
        .chars()
        .flat_map(|c| {
            if c.is_ascii_digit() {
                Some(c)
            } else {
                collate.get(&c).cloned()
            }
        })
        .collect::<Vec<char>>()
        .chunks(5)
        .map(|s| s.iter().collect())
        .collect::<Vec<String>>()
        .join(" ")
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    let alphabet = (b'a'..=b'z').map(char::from);
    let tebahpla = (b'a'..=b'z').rev().map(char::from);
    let collate = tebahpla.zip(alphabet).collect::<HashMap<_, _>>();
    cipher
        .chars()
        .flat_map(|c| {
            if c.is_ascii_digit() {
                Some(c)
            } else {
                collate.get(&c).cloned()
            }
        })
        .collect::<String>()
}
