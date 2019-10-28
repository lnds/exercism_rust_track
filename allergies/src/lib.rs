pub struct Allergies {
    score: u32,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Allergen {
    Eggs = 1,
    Peanuts = 1 << 1,
    Shellfish = 1 << 2,
    Strawberries = 1 << 3,
    Tomatoes = 1 << 4,
    Chocolate = 1 << 5,
    Pollen = 1 << 6,
    Cats = 1 << 7,
}

use Allergen::*;

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies { score }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        let val = *allergen as u32;
        self.score & val == val
       
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let mut result = vec![];
        
        if self.score & 1u32 == 1u32 {
            result.push(Eggs);
        }
        if self.score & 2u32 == 2u32 {
            result.push(Peanuts)
        }
        if self.score & 4u32 == 4u32 {
            result.push(Shellfish)
        }
        if self.score & 8u32 == 8u32 {
            result.push(Strawberries)
        }
        if self.score & 16u32 == 16u32 {
            result.push(Tomatoes)
        }
        if self.score & 32u32 == 32u32 {
            result.push(Chocolate)
        }
        if self.score & 64u32 == 64u32 {
            result.push(Pollen)
        }
        if self.score & 128u32 == 128u32 {
            result.push(Cats)
        }
        result
    }
}
