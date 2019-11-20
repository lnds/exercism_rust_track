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
    fn years_during(d: &Duration) -> f64 {
        let year_in_seconds = 365.25 * 24.0 * 3600.0 * Self::factor();
        d.seconds / year_in_seconds
    }

    fn factor() -> f64 {
        unimplemented!()
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
    fn factor() -> f64 {
        0.240_846_7
    }
}
impl Planet for Venus {
    fn factor() -> f64 {
        0.615_197_26
    }
}

impl Planet for Earth {
    fn factor() -> f64 {
        1.0
    }
}
impl Planet for Mars {
    fn factor() -> f64 {
        1.880_815_8
    }
}
impl Planet for Jupiter {
    fn factor() -> f64 {
        11.862_615
    }
}
impl Planet for Saturn {
    fn factor() -> f64 {
        29.447_498
    }
}
impl Planet for Uranus {
    fn factor() -> f64 {
        84.016_846
    }
}

impl Planet for Neptune {
    fn factor() -> f64 {
        164.79132
    }
}
