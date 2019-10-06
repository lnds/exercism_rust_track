#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Default)]
pub struct BowlingGame {
    position: usize,
    throws: [Option<u16>; 21],
}

impl BowlingGame {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.game_ended() {
            return Err(Error::GameComplete);
        }
        if self.position == 20 {
            let pre_score = self.throws[18].unwrap();
            let score = self.throws[19].unwrap();
            self.position = 21;
            match pins {
                0..=10 => match pins {
                    0..=10 if score == 10 || pre_score + score == 10 || score + pins <= 10 => {
                        self.throws[20] = Some(pins);
                        Ok(())
                    }
                    _ => Err(Error::NotEnoughPinsLeft),
                },
                _ => {
                    if score + pins <= 10 {
                        self.throws[20] = Some(pins);
                        Ok(())
                    } else {
                        Err(Error::NotEnoughPinsLeft)
                    }
                }
            }
        } else if self.position % 2 == 0 {
            match pins {
                0..=9 => {
                    self.throws[self.position] = Some(pins);
                    self.position += 1;
                    Ok(())
                }
                10 => {
                    self.throws[self.position] = Some(pins);
                    self.position += if self.position == 18 { 1 } else { 2 };
                    Ok(())
                }
                _ => Err(Error::NotEnoughPinsLeft),
            }
        } else {
            let score = self.throws[self.position - 1].unwrap();
            match score + pins {
                0..=10 => {
                    self.throws[self.position] = Some(pins);
                    self.position += 1;
                    Ok(())
                }
                _ if score == 10 => match pins {
                    0..=10 => {
                        self.throws[self.position] = Some(pins);
                        self.position += 1;
                        Ok(())
                    }
                    _ => Err(Error::NotEnoughPinsLeft),
                },
                _ => Err(Error::NotEnoughPinsLeft),
            }
        }
    }

    fn game_ended(&self) -> bool {
        (self.position == 21
            && (self.throws[18].unwrap() == 10
                || self.throws[18].unwrap() + self.throws[19].unwrap() == 10))
            || (self.position == 20
                && (self.throws[18].unwrap() < 10
                    && self.throws[18].unwrap() + self.throws[19].unwrap() < 10))
    }

    pub fn score(&self) -> Option<u16> {
        if !self.game_ended() {
            return None;
        }
        let mut frames = [0; 10];
        for (i, item) in frames.iter_mut().enumerate() {
            let j = i * 2;
            if self.throws[j].unwrap() == 10 {
                // strike
                if i < 9 {
                    if self.throws[j + 2].unwrap() < 10 {
                        *item = 10 + self.throws[j + 2].unwrap() + self.throws[j + 3].unwrap()
                    } else {
                        *item = 20 + self.throws[j + 4].unwrap()
                    }
                } else if self.throws[j + 1].unwrap() < 10 {
                    *item = 10 + self.throws[j + 1].unwrap() + self.throws[j + 2].unwrap()
                } else {
                    *item = 20 + self.throws[j + 1].unwrap()
                }
            } else if self.throws[j].unwrap() + self.throws[j + 1].unwrap() == 10 {
                // spare
                *item = 10 + self.throws[j + 2].unwrap();
            } else {
                *item = self.throws[j].unwrap() + self.throws[j + 1].unwrap();
            }
        }
        Some(frames.iter().sum())
    }
}
