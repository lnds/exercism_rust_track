use std::cmp::PartialEq;

#[derive(Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    fn inc_min(hour: i32, min: i32) -> Clock {
        let mut hour = if hour < 0 {
            24 - hour.abs() % 24
        } else {
            hour % 24
        };

            hour = if min < 0 {
                (hour + min / 60 - 1) % 24
            } else {
                (hour + (min / 60)) % 24
            };
        if hour < 0 {
            hour = 24 - hour.abs() % 24
        }
        let mut min = if min < 0 {
            60 - min.abs() % 60
        } else {
            min % 60
        };

         if min == 60 {
            hour = (hour + 1) % 24;
            min = 0;
        }
        Clock {
            hours: hour,
            minutes: min,
        }
    }

    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock::inc_min(hours, minutes)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::inc_min(self.hours, self.minutes + minutes)
    }
}

impl ToString for Clock {
    fn to_string(&self) -> String {
        format!("{:02}:{:02}", self.hours, self.minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours % 24 == other.hours % 24 && self.minutes == other.minutes
    }
}
