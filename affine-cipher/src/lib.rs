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
        Err(AffineCipherError::NotCoprime(a))
    } else {
        unimplemented!("Decode {} with the key ({}, {})", ciphertext, a, b);
    }
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
