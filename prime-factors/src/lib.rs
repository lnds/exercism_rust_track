pub fn factors(n: u64) -> Vec<u64> {
    let mut result = vec![];
    let mut num = n;
    let mut candidate = 2;
    while candidate <= num && num > 1 {
        while num % candidate == 0 {
            result.push(candidate);
            num /= candidate;
        } 
        candidate += 1;
    }
    result
}
