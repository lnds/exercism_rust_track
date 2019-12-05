pub fn get_diamond(c: char) -> Vec<String> {
    let width = usize::from(2 * (c as u8 - b'A') + 1);
    let ini = b'A';
    let end = c as u8;
    (ini..=end)
        .enumerate()
        .chain((ini..end).enumerate().rev())
        .map(|(i, c)| {
            let ch = char::from(c);
            if i == 0 {
                format!("{:^pad$}", ch, pad = width)
            } else {
                let s = format!("{}{:pad$}{}", ch, ' ', ch, pad = 2 * i - 1);
                format!("{:^pad$}", s, pad = width)
            }
        })
        .collect()
}
