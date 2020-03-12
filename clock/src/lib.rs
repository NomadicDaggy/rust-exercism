use std::fmt;

// Debug has to be derived because ...s
#[derive(Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut new_minutes = minutes;
        let mut new_hours = hours;

        if minutes >= 60 {
            new_minutes = minutes % 60;
            new_hours += minutes / 60;
        }

        Clock {
            hours: new_hours,
            minutes: new_minutes,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let mut new_minutes = self.minutes + minutes;
        let mut new_hours = self.hours;

        if new_minutes == 60 {
            new_minutes = 0;
            new_hours += 1;
        }

        Clock {
            hours: new_hours,
            minutes: new_minutes,
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}:{}",
            format!("{:0>2}", self.hours % 24),
            format!("{:0>2}", self.minutes % 60)
        )
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        let hours_match = self.hours % 24 == other.hours % 24;
        let minutes_match = self.minutes % 60 == other.minutes % 60;
        hours_match && minutes_match
    }
}
