use std::cmp::PartialOrd;
use std::ops::Add;

#[derive(PartialEq)]
enum Kind {
    Equilateral,
    Isosceles,
    Scalene,
}

use Kind::{Equilateral, Isosceles, Scalene};

pub struct Triangle(Kind);

impl Triangle {
    pub fn build<T>(sides: [T; 3]) -> Option<Triangle>
    where
        T: Copy + PartialOrd + Add<Output = T> + Default,
    {
        match (sides[0], sides[1], sides[2]) {
            (a, b, c) if a == T::default() || b == T::default() || c == T::default() => None,
            (a, b, c) if (a + b) <= c || a + c <= b || b + c <= a => None,
            (a, b, c) if a == b && b == c => Some(Triangle(Equilateral)),
            (a, b, c) if a == b && a != c || a == c && a != b || b == c && b != a => {
                Some(Triangle(Isosceles))
            }
            _ => Some(Triangle(Scalene)),
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self.0 == Equilateral
    }

    pub fn is_scalene(&self) -> bool {
        self.0 == Scalene
    }

    pub fn is_isosceles(&self) -> bool {
        self.0 == Isosceles
    }
}
