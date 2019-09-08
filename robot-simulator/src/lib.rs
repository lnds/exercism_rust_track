#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    x: i32,
    y: i32,
    d: Direction,
}

use Direction::{East, North, South, West};

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot { x, y, d }
    }

    pub fn turn_right(self) -> Self {
        Robot {
            d: match self.d {
                North => East,
                East => South,
                South => West,
                West => North,
            },
            ..self
        }
    }

    pub fn turn_left(self) -> Self {
        Robot {
            d: match self.d {
                North => West,
                East => North,
                South => East,
                West => South,
            },
            ..self
        }
    }

    pub fn advance(self) -> Self {
        match self.d {
            North => Robot {
                y: self.y + 1,
                ..self
            },
            East => Robot {
                x: self.x + 1,
                ..self
            },
            South => Robot {
                y: self.y - 1,
                ..self
            },
            West => Robot {
                x: self.x - 1,
                ..self
            },
        }
    }

    pub fn instructions(self, instructions: &str) -> Self {
        match instructions.chars().nth(0) {
            Some('R') => self.turn_right().instructions(&instructions[1..]),
            Some('L') => self.turn_left().instructions(&instructions[1..]),
            Some('A') => self.advance().instructions(&instructions[1..]),
            Some(_) => self.instructions(&instructions[1..]),
            None => self,
        }
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }
}
