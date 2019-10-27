pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let height = input.len();
    let width = input[0].len();
    let cols = (0..width)
        .map(|col| (0..height).map(|row| input[row][col]).collect::<Vec<u64>>())
        .collect::<Vec<_>>();

    input.iter().enumerate().flat_map(|(i, row)| {
        cols.iter().enumerate().flat_map(move |(j, col)| {
            let min = col.iter().min().unwrap();
            let max = row.iter().max().unwrap();
            if min == max {
                Some((i, j))
            } else {
                None
            }
        })
    }).collect()
}
