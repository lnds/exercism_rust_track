pub fn annotate(minefield: &[&str]) -> Vec<String> {
    minefield
        .iter()
        .enumerate()
        .map(|(i, row)| calc_near_mines(i, row, minefield))
        .collect()
}

fn calc_near_mines(i: usize, row: &str, minefield: &[&str]) -> String {
    let north_row = expand_row(Row::North, i, row.len(), minefield);
    let south_row = expand_row(Row::South, i, row.len(), minefield);
    let this_row = expand_row(Row::This, i, row.len(), minefield);
    row.chars()
        .enumerate()
        .map(|(j, c)| match c {
            ' ' => {
                let total = count_mines(j + 1, &this_row, &north_row, &south_row);
                if total > 0 {
                    total.to_string()
                } else {
                    " ".to_string()
                }
            }
            _ => c.to_string(),
        })
        .collect()
}

enum Row {
    This,
    North,
    South,
}

fn expand_row(row: Row, i: usize, len: usize, minefield: &[&str]) -> Vec<char> {
    match row {
        Row::This => pad_row(minefield[i]),
        Row::North => {
            if i == 0 {
                vec![' '; len + 2]
            } else {
                pad_row(minefield[i - 1])
            }
        }
        Row::South => {
            if i == minefield.len() - 1 {
                vec![' '; len + 2]
            } else {
                pad_row(minefield[i + 1])
            }
        }
    }
}

fn pad_row(row: &str) -> Vec<char> {
    std::iter::once(' ')
        .chain(row.chars())
        .chain(std::iter::once(' '))
        .collect()
}

fn count_mines(j: usize, this_row: &[char], north_row: &[char], south_row: &[char]) -> usize {
    mine(j - 1, &north_row)
        + mine(j, &north_row)
        + mine(j + 1, &north_row)
        + mine(j - 1, &this_row)
        + mine(j + 1, &this_row)
        + mine(j - 1, &south_row)
        + mine(j, &south_row)
        + mine(j + 1, &south_row)
}

fn mine(j: usize, row: &[char]) -> usize {
    match row.get(j) {
        Some(c) if *c == '*' => 1,
        _ => 0,
    }
}
