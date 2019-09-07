pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let limit = upper_bound as usize;
    let mut sieve = vec![true; limit+1];
    for i in 2..=limit {
        for j in (i+i..=limit).step_by(i) {
            sieve[j] = false;
        }
    }
    (2..=upper_bound).filter(|i| sieve[*i as usize]).collect()
}
