use std::collections::{BTreeMap, BTreeSet};

#[derive(Default)]
pub struct School {
    roster: BTreeMap<u32, BTreeSet<String>>,
}

impl School {
    pub fn new() -> School {
        Default::default()
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.roster
            .entry(grade)
            .or_insert_with(BTreeSet::default)
            .insert(student.to_string());
    }

    pub fn grades(&self) -> Vec<u32> {
        self.roster.keys().copied().collect()
    }

    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        self.roster.get(&grade).map(|v| v.iter().cloned().collect())
    }
}
