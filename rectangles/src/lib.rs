use itertools::Itertools;

pub fn count(lines: &[&str]) -> u32 {
    Rectangles::new(lines).count()
}

struct Rectangles<'a> {
    lines: &'a [&'a str],
}

impl<'a> Rectangles<'a> {
    fn new(lines: &'a [&'a str]) -> Self {
        Rectangles { lines }
    }

    fn count(&self) -> u32 {
        let vertices = self.find_vertices();
        vertices
            .iter()
            .cartesian_product(vertices.iter())
            .filter(|(c0, c1)| c0.0 < c1.0 && c0.1 < c1.1 && self.connected(c0, c1))
            .count() as u32
    }

    fn find_vertices(&self) -> Vec<(usize, usize)> {
        self.lines
            .iter()
            .enumerate()
            .flat_map(|(y, &row)| {
                row.chars()
                    .enumerate()
                    .filter(|&(_, c)| c == '+')
                    .map(move |(x, _)| (x, y))
            })
            .collect()
    }

    fn connected(&self, &(x0, y0): &(usize, usize), &(x1, y1): &(usize, usize)) -> bool {
        self.lines[y0].chars().nth(x1) == Some('+')
            && self.lines[y1].chars().nth(x0) == Some('+')
            && self.lines[y0][x0..x1]
                .chars()
                .zip(self.lines[y1][x0..x1].chars())
                .all(|(c0, c1)| (c0 == '-' || c0 == '+') && (c1 == '-' || c1 == '+'))
            && self.lines[y0..y1]
                .iter()
                .map(|row| (row.chars().nth(x0).unwrap(), row.chars().nth(x1).unwrap()))
                .all(|(c0, c1)| (c0 == '|' || c0 == '+') && (c1 == '|' || c1 == '+'))
    }
}
