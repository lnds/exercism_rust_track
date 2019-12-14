use std::fmt::{Display, Formatter, Result};

pub struct Roman {
    representation: String,
}

impl Display for Roman {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.representation)
    }
}

impl From<u32> for Roman {
    fn from(num: u32) -> Self {
        Roman {
            representation: parse_roman(num as usize),
        }
    }
}

fn parse_roman(num: usize) -> String {
    format!(
        "{}{}{}{}",
        unit(part(num, 1000), "M", "", ""),
        unit(part(num, 100), "C", "D", "M"),
        unit(part(num, 10), "X", "L", "C"),
        unit(part(num, 1), "I", "V", "X")
    )
}

fn unit(part: usize, base: &str, mid: &str, limit: &str) -> String {
    match part {
        1 | 2 | 3 => base.repeat(part),
        4 => format!("{}{}", base, mid),
        5 | 6 | 7 | 8 => format!("{}{}", mid, base.repeat(part - 5)),
        9 => format!("{}{}", base, limit),
        _ => "".to_string(),
    }
}

fn part(num: usize, digit: usize) -> usize {
    num % (digit * 10) / digit
}
