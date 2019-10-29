#[derive(Debug, PartialEq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    let mut result: Vec<u8> = vec![];
    for w in values.iter() {
        let mut v = vec![];
        let mut b = *w;
        v.insert(0, (b & 0x7f) as u8);
        b >>= 7;
        while b > 0 {
            v.insert(0, (b & 0x7f) as u8 | 0x80);
            b >>= 7;
        }
        result.append(&mut v);
    }
    result
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    let mut v = vec![];
    let mut result: Vec<u32> = vec![];
    for b in bytes.iter() {
        v.push(b);
        if *b < 0x80 {
            let num: u64 = v
                .iter()
                .fold(0u64, |acum, byte| acum << 7 | u64::from(*byte & 0x7f));
            if num <= u64::from(u32::max_value()) {
                result.push(num as u32);
            } else {
                return Err(Error::Overflow);
            }
            v.clear();
        }
    }
    if result.is_empty() {
        Err(Error::IncompleteNumber)
    } else {
        Ok(result)
    }
}
