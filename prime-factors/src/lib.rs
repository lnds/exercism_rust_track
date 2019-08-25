pub fn factors(n: u64) -> Vec<u64> {
    match n {
        0|1 => vec![],
        _ => {
            let mut f : Vec<u64> = (2..n).filter(|i| n % i == 0).take(1).collect();
            if f.is_empty() {
                vec![n]
            } else {
                f.append(&mut factors(n / f.first().unwrap()));
                f
            }
        }
    }
}
