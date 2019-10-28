use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

fn factors_sum(num: u64) -> u64 {
    (1u64..num).filter(|x| num % x == 0).sum()
}

pub fn classify(num: u64) -> Option<Classification> {
    match num {
        0 => None,
        _ => match factors_sum(num).cmp(&num) {
            Ordering::Equal => Some(Classification::Perfect),
            Ordering::Less => Some(Classification::Deficient),
            _ => Some(Classification::Abundant),
        },
    }
}
