pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let mut result : Vec<Vec<u32>> = vec![];
    (0..size).for_each(|_| {
        let mut v = Vec::new();
        v.resize(size as usize, 0);
        result.push(v);
    });
    let mut val = 1;
    let mut k: usize = 0;
    let mut l: usize = 0;
    let mut n = size as usize;
    let mut m = n;
    while k < m && l < n {
        for i in l..n {
            result[k][i] = val;
            val += 1;
        }
        k += 1;
        for i in k..m {
            result[i][n-1] = val;
            val += 1;   
        }
        n -= 1;
        if k < m {
            for i in (l..=n-1).rev() {
                result[m-1][i] = val;
                val += 1;
            }
            m -= 1;
        }
        if l < n {
            for i in (k..=m-1).rev() {
                result[i][l] = val;
                val += 1;
            }
            l += 1;
        }
    }
    result
}
