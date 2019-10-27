fn sufix_encode(n: u64, m: u64, sufix: &str) -> String {
    let a = n / m;
    let b = n % m;
    if b == 0 {
        encode(a) + " " + sufix
    } else {
        encode(a) + " " + sufix + " " + &encode(b)
    }
}

pub fn encode(n: u64) -> String {
    let digits = vec![
        "zero",
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
        "ten",
        "eleven",
        "twelve",
        "thirteen",
        "fourteen",
        "fifteen",
        "sixteen",
        "seventeen",
        "eighteen",
        "nineteen",
    ];
    let tens = vec![
        "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
    ];
    match n {
        0..=19 => digits[n as usize].to_string(),
        20 | 30 | 40 | 50 | 60 | 70 | 80 | 90 => tens[((n - 20) / 10) as usize].to_string(),
        21..=99 => encode(n / 10 * 10) + "-" + &encode(n % 10),
        100 => "one hundred".to_string(),
        101..=999 => sufix_encode(n, 100, "hundred"),
        1000..=999_999 => sufix_encode(n, 1_000, "thousand"),
        1_000_000..=999_999_999 => sufix_encode(n, 1_000_000, "million"),
        1_000_000_000..=999_999_999_999 => sufix_encode(n, 1_000_000_000, "billion"),
        1_000_000_000_000..=999_999_999_999_999 => sufix_encode(n, 1_000_000_000_000, "trillion"),
        1_000_000_000_000_000..=999_999_999_999_999_999 =>sufix_encode(n, 1_000_000_000_000_000, "quadrillion"),
        _ => sufix_encode(n, 1_000_000_000_000_000_000, "quintillion"),
    }
}
