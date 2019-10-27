/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let data = isbn
        .chars()
        .filter(|c| c.is_numeric() || *c == 'x' || *c == 'X')
        .collect::<Vec<char>>();
    if data.len() == 10 {
        0 == data.iter().enumerate().fold(0, |sum, (i, c)| match c {
            'X' | 'x' => sum + 10,
            c if c.is_numeric() => sum + c.to_digit(10).unwrap() * ((10 - i) as u32),
            _ => 1,
        }) % 11
    } else {
        false
    }
}
