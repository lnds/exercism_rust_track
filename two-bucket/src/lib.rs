use std::cmp::min;

#[derive(PartialEq, Eq, Debug)]
pub enum Bucket {
    One,
    Two,
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}

fn gcd(a: u8, b: u8) -> u8 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn pour(capacity_1: u8, capacity_2: u8, goal: u8) -> (u8, u8, u8) {
    let mut from = capacity_1;
    let mut to = 0;
    let mut step = 1;
    while from != goal && to != goal {
        let temp = min(from, capacity_2 - to);
        to += temp;
        from -= temp;
        step += 1;
        if from == goal || to == goal {
            break;
        }
        if from == 0 {
            from = capacity_1;
            step += 1;
        }
        if to == capacity_2 {
            to = 0;
            step += 1;
        }
    }
    (from, to, step)
}

/// Solve the bucket problem
pub fn solve(capacity_1: u8, capacity_2: u8, goal: u8, start_bucket: &Bucket) -> Option<BucketStats> {
    if goal > capacity_2 || goal % gcd(capacity_1, capacity_2) != 0 {
        return None;
    }
    match start_bucket {
        Bucket::One if goal == capacity_2 => Some(BucketStats {
            moves: 2,
            goal_bucket: Bucket::Two,
            other_bucket: capacity_1,
        }),
        Bucket::One if goal == capacity_1 => Some(BucketStats {
            moves: 1,
            goal_bucket: Bucket::One,
            other_bucket: 0,
        }),
        Bucket::Two if goal == capacity_1 => Some(BucketStats {
            moves: 2,
            goal_bucket: Bucket::One,
            other_bucket: capacity_2,
        }),
        Bucket::Two if goal == capacity_2 => Some(BucketStats {
            moves: 1,
            goal_bucket: Bucket::Two,
            other_bucket: 0,
        }),
        Bucket::One => {
            let (from, to, steps) = pour(capacity_1, capacity_2, goal);
            println!(
                "CAP 1 ({}, {}, {}) -> from: {}, to: {}, step: {}, bucket: {:?}, goal: {}",
                capacity_1, capacity_2, goal, from, to, steps, start_bucket, goal
            );
            Some(BucketStats {
                moves: steps,
                goal_bucket: if goal == from {
                    Bucket::One
                } else {
                    Bucket::Two
                },
                other_bucket: if goal == from { to } else { from },
            })
        }
        _ => {
            let (from, to, steps) = pour(capacity_2, capacity_1, goal);

            println!(
                "CAP 2  ({}, {}, {})  -> from: {}, to: {}, step: {}, bucket: {:?}, goal: {}",
                capacity_1, capacity_2, goal, from, to, steps, start_bucket, goal
            );

            Some(BucketStats {
                moves: steps,
                goal_bucket: if goal == from {
                    Bucket::Two
                } else {
                    Bucket::One
                },
                other_bucket: if goal == from { to } else { from },
            })
        }
    }
}
