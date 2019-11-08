use rand::{thread_rng, Rng};
use rand::distributions::*;

pub struct Robot {
    name: String,
}


impl Robot {
    pub fn new() -> Self {
        Robot{name:random_name(),}
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset_name(&mut self) {
        self.name = random_name();
    }
}


fn random_name() -> String {
    let rng = thread_rng();
    let alpha = Uniform::new_inclusive(b'A', b'Z');
    let num = Uniform::new_inclusive(b'0', b'9');
    rng.sample_iter(&alpha).take(2).chain(rng.sample_iter(&num).take(3)).map(char::from).collect()
}