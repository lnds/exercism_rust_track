pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

fn is_valid(code: &str) -> bool {
    let code: Vec<char> = code.chars().filter(|c| *c != ' ').collect();
    if code.len() <= 1 {
        return false;
    }
    if code.iter().any(|c| !c.is_numeric()) {
        return false;
    }
    0 == code.iter().rev().enumerate().fold(0, |acum, (i, c)| {
        let num = *c as u8 - b'0';
        if i % 2 == 0 {
            acum + num
        } else if num * 2 > 9 {
            acum + (num * 2 - 9)
        } else {
            acum + num * 2
        }
    }) % 10
}

impl<T> Luhn for T where T:ToString {
    fn valid_luhn(&self) -> bool {
        is_valid(&self.to_string())
    }
}
