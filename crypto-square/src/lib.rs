pub fn encrypt(input: &str) -> String {
    let input = input
        .to_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .collect::<String>();
    let x = (1..)
        .skip_while(|x| x * x < input.len() && x * (x + 1) < input.len())
        .next()
        .unwrap();
    let (r, c) = if x * x == input.len() {
        (x, x)
    } else {
        (x, x + 1)
    };
    let input = format!("{:pad$}", input, pad = r * c);
    let rectangle = input
        .chars()
        .collect::<Vec<_>>()
        .chunks(c)
        .map(|s| s.to_vec())
        .collect::<Vec<Vec<_>>>();
    let cols = (0..c)
        .map(|col| (0..r).map(|row| rectangle[row][col]).collect::<String>())
        .collect::<Vec<_>>();
    cols.join(" ")
}
