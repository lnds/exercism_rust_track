#[derive(Debug)]
pub enum Error {
    InvalidTonic(String),
    BadInterval(char),
}

pub struct Scale {
    notes: Vec<String>
}

const FLAT: [&str; 12] = [
    "F", "Bb", "Eb", "Ab", "Db", "Gb", "d", "g", "c", "f", "bb", "eb",
];
const SHARP_SCALE: [&str;12] = ["A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#"];
const FLAT_SCALE: [&str;12] = ["A", "Bb", "B", "C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab"];

impl Scale {
    pub fn new(tonic: &str, intervals: &str) -> Result<Scale, Error> {
        let scale = if FLAT.iter().any(|&x| x == tonic) { FLAT_SCALE} else { SHARP_SCALE };
        println!("tonic = {}, scale = {:?}", tonic, scale);
        if let Some(pos) =  scale.iter().position(|&x| x == tonic || x.to_uppercase() == tonic.to_uppercase()) {
            Scale::new_scale(intervals, pos, scale)
        } else {
            Err(Error::InvalidTonic(tonic.to_string()))
        }
    }

    fn new_scale(intervals: &str, pos: usize, scale: [&str;12]) -> Result<Scale, Error> {
        let mut notes = vec![];
        let mut p = pos;
        notes.push(scale[p].to_string());
        for interval in intervals.chars() {
                if interval == 'M' {
                    p = (p+2)%12;
                } else if interval == 'm' {
                    p = (p+1)%12;
                } else if interval == 'A' {
                    p = (p+3)%12;
                } else {
                    return Err(Error::BadInterval(interval))
                }
                notes.push(scale[p].to_string());
        }
        notes.pop();
        Ok(Scale { notes })
    }

    pub fn chromatic(tonic: &str) -> Result<Scale, Error> {
        Scale::new(tonic, "mmmmmmmmmmmm")
    }

    pub fn enumerate(&self) -> Vec<String> {
        self.notes.clone()
    }
}
