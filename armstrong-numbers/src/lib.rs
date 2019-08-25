fn digits(n: u32) -> Vec<u32> {
    match n {
        0 => vec![],
        x => [digits(x / 10), vec![x % 10]].concat()
    }
}

pub fn is_armstrong_number(num: u32) -> bool {
    let d = digits(num);
    let n = d.len() as u32;
    d.iter().fold(0, |acum, x| acum + x.pow(n)) == num
}
