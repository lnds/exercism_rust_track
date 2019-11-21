// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration {
    seconds: f64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { seconds: s as f64 }
    }
}

pub trait Planet {
    const F: f64;

    fn years_during(d: &Duration) -> f64 {
        let year_in_seconds = 31_557_600.0 * Self::F;
        d.seconds / year_in_seconds
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    const F: f64 = 0.240_846_7;
}
impl Planet for Venus {
    const F: f64 = 0.615_197_26;
}

impl Planet for Earth {
    const F: f64 = 1.0;
}
impl Planet for Mars {
    const F: f64 = 1.880_815_8;
}
impl Planet for Jupiter {
    const F: f64 = 11.862_615;
}
impl Planet for Saturn {
    const F: f64 = 29.447_498;
}
impl Planet for Uranus {
    const F: f64 = 84.016_846;
}

impl Planet for Neptune {
    const F: f64 = 164.79132;
}
