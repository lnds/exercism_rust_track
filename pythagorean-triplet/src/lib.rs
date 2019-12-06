use std::cmp::min;
use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let s2 = sum / 2;
    let mlimit = (f64::from(s2).sqrt().ceil() - 1.0) as u32;
    (2..=mlimit)
        .filter(|m| s2 % m == 0)
        .flat_map(|m| find_triplets(det_sm(s2 / m, m), s2, m, sum))
        .collect()
}

fn find_triplets(sm: u32, s2: u32, m: u32, sum: u32) -> HashSet<[u32; 3]> {
    (m + 1 + (m % 2)..min(2 * m, sm + 1))
        .step_by(2)
        .filter(|k| sm % k == 0 && gcd(*k, m) == 1)
        .flat_map(|k| {
            let (a,b,c) = get_triplet_from_params(s2/(k*m), k-m, m);
            if a + b + c == sum {
                Some(sorted(a, b, c))
            } else {
                None
            }
        })
        .collect()
}


fn get_triplet_from_params(d:u32, n:u32, m:u32) -> (u32, u32,u32) {
    (d * (m*m - n*n), 2 * d * m * n, d * (m*m + n*n))
}

fn det_sm(sm: u32, m: u32) -> u32 {
    if sm % 2 != 0 {
        sm
    } else {
        det_sm(sm / 2, m)
    }
}

fn gcd(a: u32, b: u32) -> u32 {
    if a == b {
        a
    } else if a > b {
        gcd(a - b, b)
    } else {
        gcd(a, b - a)
    }
}

fn sorted(a: u32, b: u32, c: u32) -> [u32; 3] {
    let mut r = [a, b, c];
    r.sort();
    r
}
