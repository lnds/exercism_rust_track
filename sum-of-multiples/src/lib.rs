use std::cmp;

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

fn coprime(a: &u32, b: &u32) -> bool {
    gcd(a, b) == 1
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
    let lcms : Vec<u32> = pairs(factors).collect::<Vec<_>>().iter().map(|w| lcm(w.0, w.1)).collect();

    println!("lcms = {:?}", lcms);
    let mut sum = 0;
    for &f in factors {
        sum += sum_mult(f, limit);
    }
    let neg = lcms.iter().fold(0, |neg, n| {
        neg + sum_mult(*n, limit)
    });
    sum  - neg
    /*
    for &f in factors {
        sum += sum_mult(f, limit);
    }
    println!("sum = {}", sum);
    println!("lcms = {:?}", lcms); 
    for w in pairs(factors).collect::<Vec<_>>() {
        if coprime(w.0, w.1) {
            let f = lcm(w.0, w.1);
            println!("w.0= {}, w.1 = {}, f = {}, coprime? {}", w.0, w.1, f, coprime(w.0, w.1));
            sum -= sum_mult(f, limit);  
       } else {
            let f = if lcm(w.0, w.1) == 0 { 0} else { w.0*w.1 / lcm(w.0, w.1) };
            println!("w.0= {}, w.1 = {}, f = {}, coprime? {}", w.0, w.1, f, coprime(w.0, w.1));
            sum -= sum_mult(f, limit);  
       }
    }
    /*
    let lcms : Vec<u32> = pairs(factors).collect::<Vec<_>>().iter().map(|w| lcm(w.0, w.1)).collect();
    for f in lcms {
        println!("f = {}", f);
        sum -= sum_mult(f, limit);
    }
    */
    println!("sum = {}", sum);
    sum
    */
}

/*
 * sum_of_multiples(150, &[5, 6, 8])
 * limit = 150, factors = [5, 6, 8]
 * expected = 4419
 * 5: 149/5 = 29 => sum(5) = (5 * 29 * 30) / 2 = 2175
 * 6: 149/6 = 24 => sum(6) = (6 * 24 * 25) / 2 = 1800
 * 8: 149/8 = 18 => sum(8) = (8 * 18 * 19) / 2 = 1368
 * total = 2175 + 3600 + 2736 = 5343
 * lcm(5, 6) = 30 => 149/30 = 4 => (30 * 4 * 5)/2 = 300
 * lcm(5, 8) = 40 => 149/40 = 3 => (40 * 3 * 4)/2 = 240
 * lcm(6, 8) => 24 => 149/24 = 3 => (24 * 6 * 7)/2 = 504
 * lcm total = 1044 => 5343 - 1044 = 4299
 */