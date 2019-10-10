#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum State {
    NotPlayed,
    Waiting(u16),
    Open,
    Spare,
    Strike,
}

impl Default for State {
    fn default() -> Self {
        State::NotPlayed
    }
}

#[derive(Default, Copy, Clone)]
struct Frame(Option<u16>, Option<u16>, State);

use State::*;

impl Frame {
    fn roll(self, pins: u16) -> Result<Frame, Error> {
        match self {
            Frame(None, None, NotPlayed) if pins < 10 => Ok(Frame(Some(pins), None, Waiting(10 - pins))),
            Frame(None, None, NotPlayed) if pins == 10 => Ok(Frame(Some(pins), None, Strike)),
            Frame(Some(score), None, Waiting(_)) if pins + score == 10 => Ok(Frame(Some(score), Some(pins), Spare)),
            Frame(Some(score), None, Waiting(_)) if pins + score < 10 => Ok(Frame(Some(score), Some(pins), Open)),
            _ => Err(Error::NotEnoughPinsLeft),
        }
    }
}

#[derive(Default)]
pub struct BowlingGame {
    frame: usize,
    frames: [Frame; 12],
}

impl BowlingGame {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.game_ended() {
            return Err(Error::GameComplete);
        }
        let frame = self.frames[self.frame];
        let result = frame.roll(pins);
        match result {
            Ok(frame) => {
                self.frames[self.frame] = frame;
                let Frame(_, _, state) = frame;
                if state == Open || state == Strike || state == Spare {
                    self.frame += 1
                }
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    fn game_ended(&self) -> bool {
        let Frame(_, _, a) = self.frames[9];
        let Frame(_, _, b) = self.frames[10];
        let Frame(_, _, c) = self.frames[11];
        match a {
            Strike => (b == Strike && c != NotPlayed) || (b == Open || b == Spare),
            Spare => b != NotPlayed,
            Open => true,
            _ => false,
        }
    }

    pub fn score(&self) -> Option<u16> {
        if !self.game_ended() {
            return None;
        }
        Some(
            self.frames
                .windows(3)
                .map(|s| match s[0] {
                    Frame(_, _, Strike) => {
                        10 + s[1].0.unwrap() + s[1].1.unwrap_or_else(|| s[2].0.unwrap())
                    }
                    Frame(_, _, Spare) => 10 + s[1].0.unwrap(),
                    Frame(Some(a), Some(b), Open) => a + b,
                    _ => 0,
                })
                .sum(),
        )
    }
}
