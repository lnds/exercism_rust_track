pub struct Luhn {
    code: String,
}

impl Luhn {
    pub fn is_valid(&self) -> bool {
        let code: Vec<char> = self.code.chars().filter(|c| *c != ' ').collect();
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
}

impl<T> From<T> for Luhn
where
    T: ToString,
{
    fn from(input: T) -> Self {
        Luhn {
            code: input.to_string(),
        }
    }
}
