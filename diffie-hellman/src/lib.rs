fn mod_pow(b: u64, e: u64, m: u64) -> u64 {
    if m == 1 {
        return 0
    }
    let mut r: u64 = 1;
    let mut b: u64 = b % m;
    let mut e = e;
    while e > 0 {
        if e % 2 == 1 {
            r = (r * b) % m
        }
        e = e >> 1;
        b = (b * b) % m;
    }
    r
}

pub fn private_key(p: u64) -> u64 {
    (1 + p) / 2 + 1
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    mod_pow(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    mod_pow(b_pub, a, p)
}
