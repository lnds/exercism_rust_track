fn is_prime(n: u32) -> bool {
    if n <= 1 {
        false
    } else if n < 4 {
        true
    } else if n % 2 == 0 {
        false
    } else if n < 9 {
        true
    } else if n % 3 == 0 {
        false
    } else {
        let nf = f64::from(n);
        let r = nf.sqrt().floor() as u32;
        let mut f = 5;
        while f <= r {
            if n % f == 0 {
                return false;
            }
            if n % (f + 2) == 0 {
                return false;
            }
            f += 6;
        }
        true
    }
}

pub fn nth(n: u32) -> u32 {
    let mut count = 0;
    let mut candidate: u32 = 1;
    loop {
        candidate += 1;
        if is_prime(candidate) {
            count += 1;
        }
        if count > n {
            break;
        }
    }
    candidate
}
