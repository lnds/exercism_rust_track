pub struct PascalsTriangle {
    row_count: u32,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle { row_count }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        (0..self.row_count).map(PascalsTriangle::calc_row).collect()
    }

    fn calc_row(n: u32) -> Vec<u32> {
        (0..=n).map(|k| PascalsTriangle::bin_coef(n, k)).collect()
    }

    fn bin_coef(n: u32, k: u32) -> u32 {
        if k == 0 {
            1
        } else if n == 0 && k > 0 {
            0
        } else {
            PascalsTriangle::bin_coef(n - 1, k - 1) + PascalsTriangle::bin_coef(n - 1, k)
        }
    }
}
