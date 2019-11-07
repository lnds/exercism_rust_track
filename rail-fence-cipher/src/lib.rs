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
        let m: i32 = self.rails as i32;
        let mut row: i32 = 0;
        let mut delta: i32 = 1;
        let mut fences = vec![String::new();m as usize];
        for c in text.chars() {
            fences[row as usize].push(c);
            row += delta;
            if row == m {
                delta = -1;
                row = m - 2;
            } else if row < 0 {
                delta = 1;
                row = 1;
            }
        }
        fences.join("")
    }

    pub fn decode(&self, cipher: &str) -> String {
        let m: i32 = self.rails as i32;
        let mut fences = vec![String::new(); m as usize];
        let mut row: i32 = 0;
        let mut delta: i32 = 1;
        for _ in 0..cipher.len() {
            fences[row as usize].push('?');
            row += delta;
            if row == m {
                delta = -1;
                row = m - 2;
            } else if row < 0 {
                delta = 1;
                row = 1;
            }
        }

        let mut iter = cipher.to_string();
        fences.iter_mut().for_each(|fence| {
            let n = fence.len();
            *fence = iter[..n].to_string();
            iter = iter[n..].to_string();
        });

        let mut result = String::new();
        row = 0;
        delta = 1;
        for _ in 0..cipher.len() {
            let fence = &mut fences[row as usize];
            result.push(fence.remove(0));
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
