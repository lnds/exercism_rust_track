use num_bigint::BigInt;
use num_traits::identities::{One, Zero};
use num_traits::pow::Pow;
use std::ops::{Add, Mul, Sub};
use std::str::FromStr;

#[derive(Debug)]
pub struct Decimal {
    mantise: BigInt,
    power: u32,
}

impl Decimal {
    pub fn try_from(input: &str) -> Option<Decimal> {
        let neg = input.starts_with('-');
        let sign = if neg {
            BigInt::from(-1i32)
        } else {
            BigInt::one()
        };
        let input = if neg { &input[1..] } else { &input[..] };
        let parts: Vec<&str> = input.split('.').collect();
        if parts.len() > 2 {
            None
        } else if parts.len() == 1 {
            let mantise = BigInt::from_str(parts[0]).ok()?;
            Some(Decimal {
                mantise: mantise * sign,
                power: 0,
            })
        } else {
            let int_part = BigInt::from_str(parts[0]).ok()?;
            let dec_part = BigInt::from_str(parts[1]).ok()?;
            let power = (parts[1].len()) as u32;
            let mult = BigInt::from(10u32).pow(&power);
            Some(Decimal {
                mantise: sign * (int_part * mult + dec_part),
                power,
            })
        }
    }
}

impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Decimal) -> Option<std::cmp::Ordering> {
        if self.power == other.power {
            self.mantise.partial_cmp(&other.mantise)
        } else if self.power < other.power {
            let a = &self.mantise * mult(other.power, self.power);
            let b = &other.mantise;
            a.partial_cmp(&b)
        } else {
            let a = &other.mantise * mult(self.power, other.power);
            let b = &self.mantise;
            b.partial_cmp(&a)
        }
    }
}

impl PartialEq for Decimal {
    fn eq(&self, other: &Decimal) -> bool {
        if self.power == other.power {
            self.mantise.eq(&other.mantise)
        } else if self.power < other.power {
            let a = &self.mantise * mult(other.power, self.power);
            let b = &other.mantise;
            a.eq(&b)
        } else {
            let a = &other.mantise * mult(self.power, other.power);
            let b = &self.mantise;
            b.eq(&a)
        }
    }
}

impl Add for Decimal {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let a = self.mantise;
        let b = other.mantise;
        if self.power == other.power {
            Self {
                mantise: a + b,
                power: self.power,
            }
        } else if self.power < other.power {
            Self {
                mantise: a * mult(other.power, self.power) + b,
                power: other.power,
            }
        } else {
            Self {
                mantise: a + b * mult(self.power, other.power),
                power: self.power,
            }
        }
    }
}

impl Sub for Decimal {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let a = self.mantise;
        let b = other.mantise;
        if self.power == other.power {
            Self {
                mantise: a - b,
                power: self.power,
            }
        } else if self.power < other.power {
            Self {
                mantise: a * mult(other.power, self.power) - b,
                power: other.power,
            }
        } else {
            Self {
                mantise: a - b * mult(self.power, other.power),
                power: self.power,
            }
        }
    }
}

impl Mul for Decimal {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let a = self.mantise;
        let b = other.mantise;
        let mut result = &a * &b;
        let mut power = self.power;
        if self.power < other.power && self.power > 0 {
            result *= mult(other.power, self.power);
            power = other.power;
        } else if self.power > other.power && other.power > 0 {
            result *= mult(self.power, other.power);
            power = self.power;
        }
        if &result % BigInt::from(10) == BigInt::zero() {
            result /= BigInt::from(10);
        }
        Self {
            mantise: result,
            power,
        }
    }
}

fn mult(pow1: u32, pow2: u32) -> BigInt {
    BigInt::from(10u32).pow(pow1 - pow2)
}
