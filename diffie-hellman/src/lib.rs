fn modular_pow(base: u64, exponent: u64, modulus: u64) -> u64 {
    let mut c: u64 = 1;
    let mut e = exponent;
    let mut b = base % modulus;
    loop {
        if e % 2 == 1 {
            c *= b;
            c %= modulus;
        }
        if e == 1 {
            return c;
        }
        e /= 2;
        b *= b;
        b %= modulus;
    }
}

pub fn private_key(p: u64) -> u64 {
    (1 + p) / 2 + 1
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    modular_pow(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    modular_pow(b_pub, a, p)
}
