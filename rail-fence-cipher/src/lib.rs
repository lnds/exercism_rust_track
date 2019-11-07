pub struct RailFence {
    rails: usize,
}

impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        RailFence {
            rails: rails as usize,
        }
    }

    pub fn encode(&self, text: &str) -> String {
        let n = text.len();
        let m: i32 = self.rails as i32;
        let mut fences = vec![vec!['\0'; n]; m as usize];
        let mut row: i32 = 0;
        let mut delta: i32 = 1;
        for (i, c) in text.chars().enumerate() {
            fences[row as usize][i] = c;
            row += delta;
            if row == m {
                delta = -1;
                row = m - 2;
            } else if row < 0 {
                delta = 1;
                row = 1;
            }
        }
        fences
            .iter()
            .map(|fence| fence.iter().filter(|&c| *c > '\0'))
            .map(|v| v.into_iter().collect::<String>())
            .collect()
    }

    pub fn decode(&self, cipher: &str) -> String {
        let m: i32 = self.rails as i32;
        let n = cipher.len();
        let mut fences = vec![vec!['\0'; n]; m as usize];
        let mut row: i32 = 0;
        let mut delta: i32 = 1;
        println!("cipher = {}", cipher);
        for i in 0..cipher.len() {
            fences[row as usize][i] = '?';
            row += delta;
            if row == m {
                delta = -1;
                row = m - 2;
            } else if row < 0 {
                delta = 1;
                row = 1;
            }
        }
        println!("fences: [{:?}]", fences);

        let mut iter = cipher.chars();
        for fence in fences.iter_mut() {
            for r in fence.iter_mut() {
                if *r == '?' {
                    *r = iter.next().unwrap();
                }
            }
        }

        let mut result = String::new();
        row = 0;
        delta = 1;
        for i in 0..cipher.len() {
            if fences[row as usize][i] > '\0' {
                result.push(fences[row as usize][i]);
            }
            row += delta;
            if row == m {
                delta = -1;
                row = m - 2;
            } else if row < 0 {
                delta = 1;
                row = 1;
            }
        }
        result
    }
}
