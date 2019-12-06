use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut result = HashSet::new();
    let s2 = sum / 2;
    let mlimit = (f64::from(s2).sqrt().ceil() - 1.0) as u32;
    for m in 2..=mlimit {
        if s2 % m == 0 {
            let sm = det_sm(s2 / m, m);
            let  k = m + 1 + (m % 2);
            result = result.union(&find_triplets(sm, k, m, sum, HashSet::new())).cloned().collect();
            /*
            while k < 2 * m && k <= sm {
                if sm % k == 0 && gcd(k, m) == 1 {
                    let d = s2 / (k * m);
                    let n = k - m;
                    let a = d * (m * m - n * n);
                    let b = 2 * d * m * n;
                    let c = d * (m * m + n * n);
                    if a + b + c == sum {
                        result.insert(sorted(a, b, c));
                    }
                }
                k += 2;
            }
            */
        }
    }
    result
}

fn find_triplets(sm: u32, k: u32, m: u32, sum: u32, result: HashSet<[u32; 3]>) -> HashSet<[u32; 3]> {
    if k >= 2*m || k > sm {
        return result;
    } else {
        let s2 = sum / 2;
        if sm % k == 0 && gcd(k, m) == 1 {
            let d = s2 / (k * m);
            let n = k - m;
            let a = d * (m * m - n * n);
            let b = 2 * d * m * n;
            let c = d * (m * m + n * n);
            if a + b + c == sum {
                return find_triplets(sm, k+2, m, sum, result.union(&sorted(a,b,c)).cloned().collect());
            }
        } 
        return find_triplets(sm, k+2, m, sum, result);
    }
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

fn sorted(a: u32, b: u32, c: u32) -> HashSet<[u32; 3]> {
    let mut r = [a, b, c];
    r.sort();
    let mut result = HashSet::new();
    result.insert(r);
    result
}
