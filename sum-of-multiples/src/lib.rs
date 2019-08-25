fn pairs<I: IntoIterator>(x: I) -> impl Iterator<Item = (I::Item, I::Item)>
where
    I::Item: Clone,
    I: Copy,
{
    x.into_iter()
        .enumerate()
        .flat_map(move |t| std::iter::repeat(t.1).zip(x.into_iter().skip(t.0 + 1)))
}

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

fn lcm(a: &u32, b: &u32) -> u32 {
    if *a == 0 || *b == 0 {
        0
    } else {
        (*a * *b) / gcd(a, b)
    }
}

fn sum_mult(n: u32, limit: u32) -> u32 {
    if n == 0 {
        0
    } else {
        let p = (limit - 1) / n;
        n * (p * (p + 1)) / 2
    }
}

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let lcms: Vec<u32> = pairs(factors)
        .collect::<Vec<_>>()
        .iter()
        .map(|w| lcm(w.0, w.1))
        .collect();

    let sum = factors.iter().fold(0, |acum, f| acum + sum_mult(*f, limit));
    let neg = lcms.iter().fold(0, |neg, n| neg + sum_mult(*n, limit));

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

    sum - neg + pos
}
