// Sum of multiples
// Author: Eduardo Díaz
// Uses math formulas to speed up calculations


// create pairs from a vector
fn pairs<I: IntoIterator>(x: I) -> impl Iterator<Item = (I::Item, I::Item)>
where
    I::Item: Clone,
    I: Copy,
{
    x.into_iter()
        .enumerate()
        .flat_map(move |t| std::iter::repeat(t.1).zip(x.into_iter().skip(t.0 + 1)))
}


// greatest common divisor of 2 numbers
fn gcd(a: &u32, b: &u32) -> u32 {
    if *a == 0 || *b == 0 {
        0
    } else if *a == *b {
        *a
    } else if a > b {
        gcd(&(a - b), b)
    } else {
        gcd(a, &(b - a))
    }
}

// less commons multiple of 2 numbers
fn lcm(a: &u32, b: &u32) -> u32 {
    if *a == 0 || *b == 0 {
        0
    } else {
        (*a * *b) / gcd(a, b)
    }
}

// gauss formula for sum
fn sum_mult(n: u32, limit: u32) -> u32 {
    if n == 0 {
        0
    } else {
        let p = (limit - 1) / n;
        n * (p * (p + 1)) / 2
    }
}

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    if factors.len() > 3 {
        // this is clasical solution, iterating over all numbers until limit
        // but a faster solution will use gauss formula (see else)
        // with more time will improve this and not else case will be needed
        (0..limit).filter(|x| factors.iter().fold(false, |r, f| r || x % f == 0 )).sum()
    }
    else {
        // uses math to solve with few iterations
        // uses gauss formula to sum all factors
        let sum = factors.iter().fold(0, |acum, f| acum + sum_mult(*f, limit));

        // we must substract all lcms of factors
        let lcms: Vec<u32> = pairs(factors)
            .collect::<Vec<_>>()
            .iter()
            .map(|w| lcm(w.0, w.1))
            .collect();
        let neg = lcms.iter().fold(0, |neg, n| neg + sum_mult(*n, limit));

        // but if we have coprime factors we must correct
        let mut add_lcms: Vec<u32> = pairs(&lcms)
            .collect::<Vec<_>>()
            .iter()
            .map(|w| lcm(w.0, w.1))
            .collect();
        add_lcms.sort();
        add_lcms.dedup();
        let pos = add_lcms
            .iter()
            .fold(0, |acum, n| acum + sum_mult(*n, limit));

        // finally the result    
        sum - neg + pos
    } 
}
