/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

const M: i32 = 26;

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if !coprime(a, M) {
        Err(AffineCipherError::NotCoprime(a))
    } else {
        Ok(plaintext
            .to_lowercase()
            .chars()
            .flat_map(|c| {
                if c.is_alphabetic() {
                    let c = i32::from(c as u8 - b'a');
                    let enc = (c * a + b) % M;
                    Some(char::from(enc as u8 + b'a'))
                } else if c.is_numeric() {
                    Some(c)
                } else {
                    None
                }
            })
            .collect::<Vec<char>>()
            .chunks(5)
            .map(|s| s.iter().collect())
            .collect::<Vec<String>>()
            .join(" "))
    }
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if !coprime(a, M) {
        return Err(AffineCipherError::NotCoprime(a));
    }
    let an = inverse(a, M);
    let ad = inverse_additive(b, M);
    Ok(ciphertext
        .bytes()
        .flat_map(|x| {
            if x >= b'a' && x <= b'z' {
                let c = (i32::from(x - b'a') * an + ad * an) % M;
                Some(char::from(c as u8 + b'a'))
            } else if x >= b'0' && x <= b'9' {
                Some(x as char)
            } else {
                None
            }
        })
        .collect::<String>())
}

fn coprime(a: i32, b: i32) -> bool {
    gcd(a, b) == 1
}

fn gcd(a: i32, b: i32) -> i32 {
    if a == 0 || b == 0 {
        0
    } else if a == b {
        a
    } else if a > b {
        gcd(a - b, b)
    } else {
        gcd(a, b - a)
    }
}

fn inverse(a: i32, m: i32) -> i32 {
    for n in 1..m {
        if (a * n) % m == 1 {
            return n;
        }
    }
    0
}

fn inverse_additive(b: i32, m: i32) -> i32 {
    for i in 1..m {
        if (i + b) % m == 0 {
            return i;
        }
    }
    0
}
