use std::collections::HashMap;

pub fn rotate(input: &str, key: i8) -> String {
    let alphabet = (b'a'..=b'z').map(char::from);
    let mut cipher = (b'a'..=b'z').map(char::from).collect::<Vec<char>>();
    if key > 0 {
        cipher.rotate_left(key as usize);
    } else {
        cipher.rotate_right(key.abs() as usize);
    }
    let collate = alphabet.zip(cipher).collect::<HashMap<_, _>>();
    input
        .chars()
        .flat_map(|c| {
            if !c.is_ascii_alphabetic() {
                Some(c)
            } else if c.is_ascii_lowercase() {
                collate.get(&c).cloned()
            } else {
                collate
                    .get(&c.to_ascii_lowercase())
                    .map(|c| c.to_ascii_uppercase())
            }
        })
        .collect::<String>()
}
