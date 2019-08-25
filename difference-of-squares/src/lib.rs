pub fn square_of_sum(n: u32) -> u32 {
    let s = n * (n + 1);
    (s * s) / 4
}

pub fn sum_of_squares(n: u32) -> u32 {
    let s = (2 * n + 1) * (n + 1) * n;
    s / 6
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
