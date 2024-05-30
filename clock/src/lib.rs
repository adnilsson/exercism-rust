use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    minutes: u16,
}

const MAX_MINUTES: u16 = 24 * 60;

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self { minutes: 0 }.add_minutes(60 * hours + minutes)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let minutes_sum = minutes + self.minutes as i32;
        Self {
            minutes: minutes_sum.rem_euclid(MAX_MINUTES as i32) as u16,
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.minutes / 60, self.minutes % 60)
    }
}
