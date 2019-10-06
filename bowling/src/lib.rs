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
        if self.position % 2 == 0 {
            match pins {
                0..=9 if self.position == 20 => {
                    let pre_score = self.throws[self.position - 2].unwrap();
                    let score = self.throws[self.position - 1].unwrap();
                    match pins {
                        0..=10 if score == 10 => Ok({
                            self.throws[self.position] = Some(pins);
                            self.position += 1;
                        }),
                        0..=10 if pre_score + score == 10 => Ok({
                            // spare
                            self.throws[self.position] = Some(pins);
                            self.position += 1;
                        }),
                        0..=10 if score + pins <= 10 => Ok({
                            self.throws[self.position] = Some(pins);
                            self.position += 1;
                        }),
                        _ => Err(Error::NotEnoughPinsLeft),
                    }
                }
                0..=9 => Ok({
                    self.throws[self.position] = Some(pins);
                    self.position += 1;
                }),
                10 if self.position == 20 => {
                    let pre_score = self.throws[self.position - 2].unwrap();
                    let score = self.throws[self.position - 1].unwrap();
                    match pins {
                        0..=10 if score == 10 => Ok({
                            self.throws[self.position] = Some(pins);
                            self.position += 1;
                        }),
                        0..=10 if pre_score + score == 10 => Ok({
                            // spare
                            self.throws[self.position] = Some(pins);
                            self.position += 1;
                        }),
                        0..=10 if score + pins <= 10 => Ok({
                            self.throws[self.position] = Some(pins);
                            self.position += 1;
                        }),
                        _ => Err(Error::NotEnoughPinsLeft),
                    }
                }
                10 => Ok({
                    self.throws[self.position] = Some(pins);
                    if self.position == 18 {
                        self.position += 1
                    } else {
                        self.position += 2;
                    }
                }),
                _ => {
                    if self.position < 20 {
                        Err(Error::NotEnoughPinsLeft)
                    } else {
                        let score = self.throws[self.position - 1].unwrap();
                        match score + pins {
                            0..=10 => Ok({
                                self.throws[self.position] = Some(pins);
                                self.position += 1;
                            }),
                            _ => Err(Error::NotEnoughPinsLeft),
                        }
                    }
                }
            }
        } else {
            let score = self.throws[self.position - 1].unwrap();
            match score + pins {
                0..=10 => Ok({
                    self.throws[self.position] = Some(pins);
                    self.position += 1;
                }),
                _ => {
                    if self.position < 19 {
                        Err(Error::NotEnoughPinsLeft)
                    } else {
                        if score == 10 {
                            match pins {
                                0..=10 => Ok({
                                    self.throws[self.position] = Some(pins);
                                    self.position += 1;
                                }),
                                _ => Err(Error::NotEnoughPinsLeft),
                            }
                        } else {
                            Err(Error::NotEnoughPinsLeft)
                        }
                    }
                }
            }
        }
    }

    fn game_ended(&self) -> bool {
        let result = (self.position == 21
            && (self.throws[18].unwrap() == 10
                || self.throws[18].unwrap() + self.throws[19].unwrap() == 10))
            || (self.position == 20
                && (self.throws[18].unwrap() < 10
                    && self.throws[18].unwrap() + self.throws[19].unwrap() < 10));
        result
    }

    pub fn score(&self) -> Option<u16> {
        println!(
            "score, self.position = {}, throws = {:?}",
            self.position, self.throws
        );
        if !self.game_ended() {
            return None;
        }
        let mut frames = [0; 10];
        for i in 0..10 {
            let j = i * 2;
            if self.throws[j].unwrap() == 10 {
                // strike
                if i < 9 {
                    if self.throws[j + 2].unwrap() < 10 {
                        frames[i] = 10 + self.throws[j + 2].unwrap() + self.throws[j + 3].unwrap()
                    } else {
                        frames[i] = 20 + self.throws[j + 4].unwrap()
                    }
                }
                else {
                    if self.throws[j + 1].unwrap() < 10 {
                        frames[i] = 10 + self.throws[j + 1].unwrap() + self.throws[j + 2].unwrap()
                    } else {
                        frames[i] = 20 + self.throws[j + 1].unwrap()
                    }
                }
            } else if self.throws[j].unwrap() + self.throws[j + 1].unwrap() == 10 {
                // spare
                frames[i] = 10 + self.throws[j + 2].unwrap();
            } else {
                frames[i] = self.throws[j].unwrap() + self.throws[j + 1].unwrap();
            }
        }
        return Some(frames.iter().sum());
    }
}
