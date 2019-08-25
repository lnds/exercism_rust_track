pub fn factors(n: u64) -> Vec<u64> {
    match n {
        0|1 => vec![],
        _ => {
            let  f : Vec<u64> = (2..n).filter(|i| n % i == 0).take(1).collect();
            if f.is_empty() {
                vec![n]
            } else {
                [&f[..], &factors(n / f[0])].concat()
            }
        }
    }
}
