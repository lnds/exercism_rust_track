use std::collections::HashMap;

#[derive(Default)]
pub struct School {
    roster: HashMap<u32, Vec<String>>,
}

impl School {
    pub fn new() -> School {
        School {
            roster: HashMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.roster
            .entry(grade)
            .and_modify(|v| v.push(student.to_string()))
            .or_insert_with(|| vec![student.to_string()]);
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut grades: Vec<u32> = self.roster.keys().copied().collect();
        grades.sort();
        grades
    }

    // If grade returned an `Option<&Vec<String>>`,
    // the internal implementation would be forced to keep a `Vec<String>` to lend out.
    // By returning an owned vector instead,
    // the internal implementation is free to use whatever it chooses.
    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        let item = self.roster.get(&grade);
        item.map(|v| {
            let mut vs = v.clone();
            vs.sort();
            vs
        })
    }
}
