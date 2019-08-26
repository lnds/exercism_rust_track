pub fn collatz(n: u64) -> Option<u64> {
    match n {
        0 => None,
        _ =>  {
            let mut c = n;
            let mut count = 0;
            while c > 1 {
                if c % 2 == 0 {
                    c /= 2
                } else {
                    c = c * 3 + 1
                }
                count += 1;
            }
            Some(count)
        }
    }
}
