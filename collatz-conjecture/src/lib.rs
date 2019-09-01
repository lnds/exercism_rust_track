pub fn collatz(n: u64) -> Option<u64> {
    match n {
        0 => None,
        1 => Some(0),
        n if n % 2 == 0 => collatz(n / 2).map(|r| r + 1),
        n => collatz(n * 3 + 1).map(|r| r + 1),
    }
}
