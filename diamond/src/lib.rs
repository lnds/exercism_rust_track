pub fn get_diamond(c: char) -> Vec<String> {
    let width = usize::from(2 * (c as u8 - b'A') + 1);
    (b'A'..=(c as u8))
        .enumerate()
        .chain((b'A'..(c as u8)).enumerate().rev())
        .map(|(i, c)| pad_char(char::from(c), i, width))
        .collect()
}

fn pad_char(c: char, i: usize, width: usize) -> String {
    if i == 0 {
        format!("{:^pad$}", c, pad = width)
    } else {
        let result = format!("{}{:pad$}{}", c, ' ', c, pad = 2 * i - 1);
        format!("{:^pad$}", result, pad = width)
    }
}
