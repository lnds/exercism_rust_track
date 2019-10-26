use std::collections::BTreeSet;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Palindrome {
    value: u64,
    factors: BTreeSet<(u64, u64)>,
}

impl Palindrome {
    pub fn new(a: u64, b: u64) -> Palindrome {
        Palindrome {
            value: a * b,
            factors: [(a, b)].iter().copied().collect(),
        }
    }

    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn insert(&mut self, a: u64, b: u64) {
        if !self.factors.contains(&(b, a)) {
            self.factors.insert((a, b));
        }
    }
}

fn is_palindrome(num: u64) -> bool {
    let mut reversed = 0;
    let mut n = num;
    while n > 0 {
        reversed = 10 * reversed + n % 10;
        n /= 10;
    }
    reversed == num
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut smallest = None;
    let mut largest = None;
    for x in min..=max {
        for y in min..=max {
            let value = x * y;
            if is_palindrome(value) {
                match smallest {
                    None => smallest = Some(Palindrome::new(x, y)),
                    Some(ref mut palindrome) => {
                        if value == palindrome.value() {
                            palindrome.insert(x, y)
                        } else if value < palindrome.value() {
                            smallest = Some(Palindrome::new(x, y))
                        }
                    }
                }
                match largest {
                    None => largest = Some(Palindrome::new(x, y)),
                    Some(ref mut palindrome) => {
                        if value == palindrome.value() {
                            palindrome.insert(x, y)
                        } else if value > palindrome.value() {
                            largest = Some(Palindrome::new(x, y))
                        }
                    }
                }
            }
        }
    }
    match (smallest, largest) {
        (None, None) => None,
        (None, _) => None,
        (_, None) => None,
        (Some(p1), Some(p2)) => Some((p1.clone(), p2.clone())),
    }
}
