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
        let mut fences = vec![String::new();self.rails];
        for (c, row) in text.chars().zip(zigzag(self.rails)) {
            fences[row].push(c);
        }
        fences.join("")
    }

    pub fn decode(&self, cipher: &str) -> String {
        let mut fences_len = vec![0usize; self.rails];
        for (_, row) in cipher.chars().zip(zigzag(self.rails)) {
            fences_len[row as usize] += 1;
        }
        
        let mut iter = fences_len.iter();
        let mut fences = vec![String::new(); self.rails];
        let mut i = 0;
        fences.iter_mut().for_each(|fence| {
            let n = iter.next().unwrap();
            *fence = cipher[i..i+*n].to_string();
            i += *n;
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