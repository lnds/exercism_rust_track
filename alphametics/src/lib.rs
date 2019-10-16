use permutohedron::Heap;
use std::collections::HashMap;

type Map = HashMap<char, u8>;

fn parse(input: &str) -> Option<(Vec<Vec<char>>, Vec<char>)> {
    let mut input = input.splitn(2, "==");
    let left = input.next()?;
    let right = input.next()?.trim().chars().collect();
    let left = left
        .split('+')
        .map(|num| num.trim().chars().collect())
        .collect();
    Some((left, right))
}

fn value(num: &[char], digits: &Map) -> Option<u64> {
    if num.is_empty() {
        None
    } else {
        let f = digits.get(&num[0])?;
        if *f == 0 {
            None
        } else {
        num[1..].iter().fold(Some(u64::from(*f)), |acc, d| {
            let v = u64::from(*digits.get(d)?);
            Some(acc.unwrap() * 10 + v)
        })
        }
    }
}

fn check(left: &[Vec<char>], right: &[char], digits: &Map) -> bool {
    let a = left
        .iter()
        .map(|num| value(num, &digits))
        .sum::<Option<u64>>();
    match a {
        None => false,
        Some(l) => match value(&right, &digits) {
            Some(r) => r == l,
            _ => false,
        },
    }
}

pub fn solve(input: &str) -> Option<Map> {
    let (left, right) = parse(input)?;
    let mut ret = left
        .concat()
        .iter()
        .chain(right.iter())
        .map(|c| (*c, 0u8))
        .collect::<Map>();
    let mut digits = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let heap = Heap::new(&mut digits);
    for guess in heap {
        for (i, v) in ret.values_mut().enumerate() {
            *v = guess[i];
        }
        if check(&left, &right, &ret) {
            return Some(ret);
        }
    }
    None
}
