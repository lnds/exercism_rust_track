use num_bigint::{BigInt, BigUint, Sign};
use std::ops::{Add, Mul, Sub};
use std::str::FromStr;
use num_traits::pow::Pow;

/// Type implementing arbitrary-precision decimal arithmetic
#[derive(Debug, PartialEq, PartialOrd)]
pub struct Decimal {
    mantise: BigInt,
    power: u32,
}

impl Decimal {
    pub fn try_from(input: &str) -> Option<Decimal> {
        println!("try_from({})", input);
        let parts: Vec<&str> = input.split('.').collect();
        let result = if parts.len() > 2 {
            None
        } else if parts.len() == 1 {
            match BigInt::from_str(parts[0]) {
                Ok(mantise) => Some(Decimal {
                    mantise,
                    power: 0,
                }),
                _ => None,
            }
        } else {
            match BigInt::from_str(parts[0]) {
                Ok(int_part) => match BigInt::from_str(parts[1]) {
                    Ok(dec_part) => {
                        let power = (parts[1].len()) as u32;
                        let mult = BigInt::from(10u32).pow(&power);
                        let sign = if input.starts_with("-") { BigInt::from(-1i32) } else { BigInt::from(1i32)};
                        Some(Decimal {
                        mantise: int_part*mult*sign + dec_part,
                        power,
                    })},
                    _ => None,
                },
                _ => None,
            }
        };
        println!("-> {:?}", result);
        result
    }
}

impl Add for Decimal {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if self.power == other.power {
            let a = self.mantise;
            let b = other.mantise;
            let result = a + b;
            Self {
                mantise: result,
                power: self.power,
            }
        } else if self.power < other.power {
            let mult = BigInt::from(10u32).pow(other.power-self.power);
            let a = self.mantise * mult;
            let b = other.mantise;
            let result = a + b;
            Self {
                mantise: result,
                power: other.power,
            }
        } else {
            let mult = BigInt::from(10u32).pow(self.power-other.power);
            let a = self.mantise;
            let b = other.mantise * mult;
            let result = a + b;
            Self {
                mantise: result,
                power: self.power,
            } 
        }
    }
}

impl Sub for Decimal {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        if self.power == other.power {
            let a = self.mantise;
            let b = other.mantise;
            let result = a - b;
            Self {
                mantise: result,
                power: self.power,
            }
        } else if self.power < other.power {
            let mult = BigInt::from(10u32).pow(other.power-self.power);
            let a = self.mantise * mult;
            let b = other.mantise;
            let result = a - b;
            Self {
                mantise: result,
                power: other.power,
            }
        } else {
            let mult = BigInt::from(10u32).pow(self.power-other.power);
            let a = self.mantise;
            let b = other.mantise * mult;
            let result = a - b;
            Self {
                mantise: result,
                power: self.power,
            } 
        }
    }
}

impl Mul for Decimal {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        if self.power == other.power {
            let a = self.mantise;
            let b = other.mantise;
            let result = a * b;
            Self {
                mantise: result,
                power: self.power * self.power,
            }
        } else if self.power < other.power {
            let mult = BigInt::from(10u32).pow(other.power-self.power);
            let a = self.mantise * mult;
            let b = other.mantise;
            let result = a * b;
            Self {
                mantise: result,
                power: other.power*other.power,
            }
        } else {
            let mult = BigInt::from(10u32).pow(self.power-other.power);
            let a = self.mantise;
            let b = other.mantise * mult;
            let result = a * b;
            Self {
                mantise: result,
                power: self.power*self.power,
            } 
        }
    }
}
