#[derive(Debug, PartialEq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if span > string_digits.len() {
        return Err(Error::SpanTooLong);
    }

    for c in string_digits.chars() {
        if !c.is_digit(10) {
            return Err(Error::InvalidDigit(c));
        }
    }
    if span == 0 {
        return Ok(1);
    }
    let digits:Vec<u64> = string_digits.chars().map(|c| u64::from(c.to_digit(10).unwrap())).collect();
    Ok(digits.windows(span).map(|w| w.iter().product()).max().unwrap())
}
