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

fn part(num: usize, digit: usize) -> usize {
    num % (digit * 10) / digit
}



fn parse_roman(num: usize) -> String {
    let unit = ["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"];
    let tens = ["", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"];
    let hundreds = ["", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"];
    let thousands = ["", "M", "MM", "MMM"];
    format!(
        "{}{}{}{}",
        thousands[part(num, 1000)],
        hundreds[part(num, 100)],
        tens[part(num, 10)],
        unit[part(num, 1)]
    )
}
