use std::cmp::PartialOrd;
use std::ops::Add;

enum TriangleType<T> {
    Equilateral(T),
    Isosceles(T, T),
    Scalene(T, T, T),
}

use TriangleType::{Equilateral, Isosceles, Scalene};

pub struct Triangle<T> {
    triangle: TriangleType<T>,
}

impl<T: PartialEq + PartialOrd + Add<Output = T> + Copy + Default> Triangle<T> {
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        let a: T = sides[0];
        let b: T = sides[1];
        let c: T = sides[2];
        if a == T::default()
            || b == T::default()
            || c == T::default()
            || (a + b) <= c
            || a + c <= b
            || b + c <= a
        {
            None
        } else {
            Some(if a == b && b == c {
                Triangle {
                    triangle: Equilateral(a),
                }
            } else if a == b && a != c {
                Triangle {
                    triangle: Isosceles(a, c),
                }
            } else if a == c && a != b {
                Triangle {
                    triangle: Isosceles(a, b),
                }
            } else if b == c && b != a {
                Triangle {
                    triangle: Isosceles(b, a),
                }
            } else {
                Triangle {
                    triangle: Scalene(a, b, c),
                }
            })
        }
    }

    pub fn is_equilateral(&self) -> bool {
        match self.triangle {
            Equilateral(_) => true,
            _ => false,
        }
    }

    pub fn is_scalene(&self) -> bool {
        match self.triangle {
            Scalene(_, _, _) => true,
            _ => false,
        }
    }

    pub fn is_isosceles(&self) -> bool {
        match self.triangle {
            Isosceles(_, _) => true,
            _ => false,
        }
    }
}
