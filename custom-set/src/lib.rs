
#[derive(Debug, PartialEq)]
pub struct CustomSet<T> {
    data: Vec<T>,
}

impl<T: Copy+Ord> CustomSet<T> {
    pub fn new(input: &[T]) -> Self {
        let mut data = Vec::from(input);
        data.sort();
        data.dedup();
        CustomSet {
            data
        }
    }

    pub fn contains(&self, element: &T) -> bool {
        self.data.contains(element)
    }

    pub fn add(&mut self, element: T) {
        if !self.contains(&element) {
            self.data.push(element);
            self.data.sort()
        }
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.data.iter().all(|x| other.contains(x))
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        !self.data.iter().any(|x| other.data.contains(x))
    }

    pub fn intersection(&self, other: &Self) -> Self {
        let data : Vec<T> = self.data.iter().filter(|e| other.contains(e)).copied().collect();
        CustomSet::new(&data)
    }

    pub fn difference(&self, other: &Self) -> Self {
        let data : Vec<T> = self.data.iter().filter(|e| !other.contains(e)).copied().collect();
        CustomSet::new(&data)
    }

    pub fn union(&self, other: &Self) -> Self {
        let data : Vec<T> = self.data.iter().chain(other.data.iter()).copied().collect();
        CustomSet::new(&data)
    }
}
