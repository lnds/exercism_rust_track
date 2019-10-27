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
    match n {
        0..=19 => digits[n as usize].to_string(),
        20 => "twenty".to_string(),
        21..=29 => "twenty-".to_string() + digits[(n % 10) as usize],
        30 => "thirty".to_lowercase(),
        31..=39 => "thirty-".to_string() + digits[(n % 10) as usize],
        40 => "forty".to_string(),
        41..=49 => "forty-".to_string() + digits[(n % 10) as usize],
        50 => "fifty".to_string(),
        51..=59 => "fifty-".to_string() + digits[(n % 10) as usize],
        60 => "sixty".to_string(),
        61..=69 => "sixty-".to_string() + digits[(n % 10) as usize],
        70 => "seventy".to_string(),
        71..=79 => "seventy-".to_string() + digits[(n % 10) as usize],
        80 => "eighty".to_string(),
        81..=89 => "eighty-".to_string() + digits[(n % 10) as usize],
        90 => "ninety".to_string(),
        91..=99 => "ninety-".to_string() + digits[(n % 10) as usize],
        100 => "one hundred".to_string(),
        101..=999 => {
            let cent = n / 100;
            let ucent = n % 100;
            if ucent == 0 {
                encode(cent) + " hundred"
            } else {
                encode(cent) + " hundred " + &encode(ucent)
            }
        }
        1000 => "one thousand".to_string(),
        1001..=999_999 => {
            let th = n / 1000;
            let uth = n % 1000;
            if uth == 0 {
                encode(th) + " thousand"
            } else {
                encode(th) + " thousand " + &encode(uth)
            }
        }
        1_000_000 => "one million".to_string(),
        1_000_001..=999_999_999 => {
            let m = n / 1_000_000;
            let um = n % 1_000_000;
            if um == 0 {
                encode(m) + " million"
            } else {
                encode(m) + " million " + &encode(um)
            }
        }
        1_000_000_000 => "one billion".to_string(),
        1_000_000_001..=999_999_999_999 => {
            let b = n / 1_000_000_000;
            let ub = n % 1_000_000_000;
            if ub == 0 {
                encode(b) + " billion"
            } else {
                encode(b) + " billion " + &encode(ub)
            }
        }
        1_000_000_000_000 => "one trillion".to_string(),
        1_000_000_000_001..=999_999_999_999_999 => {
            let t = n / 1_000_000_000_000;
            let ut = n % 1_000_000_000_000;
            if ut == 0 {
                encode(t) + " trillion"
            } else {
                encode(t) + " trillion " + &encode(ut)
            }
        }
        1_000_000_000_000_000 => "one quadrillion".to_string(),
        1_000_000_000_000_001..=999_999_999_999_999_999 => {
            let q = n / 1_000_000_000_000_000;
            let uq = n % 1_000_000_000_000_000;
            if uq == 0 {
                encode(q) + " quadrillion"
            } else {
                encode(q) + " quadrillion " + &encode(uq)
            }
        }
        1_000_000_000_000_000_000 => "one quintillion".to_string(),
        _ => {
            let q = n / 1_000_000_000_000_000_000;
            let uq = n % 1_000_000_000_000_000_000;
            if uq == 0 {
                encode(q) + " quintillion"
            } else {
                encode(q) + " quintillion " + &encode(uq)
            }
        }
    }
    .trim()
    .to_string()
}
