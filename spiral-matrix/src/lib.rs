pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let mut result = vec![vec![0; size as usize]; size as usize];
    let mut val = 1..;
    let mut k: usize = 0;
    let mut l: usize = 0;
    let mut n = size as usize;
    let mut m = n;
    while k < m && l < n {
        for i in l..n {
            result[k][i] = val.next().unwrap();
        }
        k += 1;
        for r in result.iter_mut().take(m).skip(k) {
            r[n-1] = val.next().unwrap();
        }
        n -= 1;
        if k < m {
            for i in (l..n).rev() {
                result[m-1][i] = val.next().unwrap();
            }
            m -= 1;
        }
        if l < n {
            for i in (k..m).rev() {
                result[i][l] = val.next().unwrap();
            }
            l += 1;
        }
    }
    result
}
