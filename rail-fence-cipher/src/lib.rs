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
        let mut fences = vec![String::new();m as usize];
        for (c, row) in text.chars().zip(zigzag(self.rails)) {
            fences[row].push(c);
        }
        fences.join("")
    }

    pub fn decode(&self, cipher: &str) -> String {
        let m = self.rails as i32;
        let mut fences = vec![String::new(); m as usize];
        for (_, row) in cipher.chars().zip(zigzag(self.rails)) {
            fences[row as usize].push('?');
        }

        let mut iter = cipher.to_string();
        fences.iter_mut().for_each(|fence| {
            let n = fence.len();
            *fence = iter[..n].to_string();
            iter = iter[n..].to_string();
        });

        let mut result = String::new();
        for (_, row) in cipher.chars().zip(zigzag(self.rails)) {
            let fence = &mut fences[row as usize];
            result.push(fence.remove(0));
        }
        result
    }
}


fn zigzag(n: usize) -> impl Iterator<Item = usize> {
    (0..n - 1).chain((1..n).rev()).cycle()
}