pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let height = input.len();
    let width = input[0].len();
    let cols = (0..width)
        .map(|col| (0..height).map(|row| input[row][col]).collect::<Vec<u64>>())
        .collect::<Vec<_>>();

    input.iter().enumerate().flat_map(|(i, row)| {
        cols.iter().enumerate().flat_map(move |(j, col)| {
            let min = col.iter().fold(None, |min, elem| match min {
                None => Some((elem, j)),
                Some((m, _)) if elem < m => Some((elem, j)),
                min => min,
            });
            let max = row.iter().fold(None, |max, elem| match max {
                None => Some((elem, i)),
                Some((m, _)) if elem > m => Some((elem, i)),
                max => max,
            });
            match (max, min) {
                (Some((a, r)), Some((b, c))) if *a == *b => Some((r, c)),
                _ => None,
            }
        })
    }).collect()
}
