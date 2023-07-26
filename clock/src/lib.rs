use std::fmt;

const MINUTES_IN_AN_HOUR: i32 = 60;
const HOURS_IN_A_DAY: i32 = 24;

#[derive(Debug, Eq, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut valid_minutes = minutes;
        let mut valid_hours = hours;
        while valid_minutes < 0 {
            let minus_hours = valid_minutes/MINUTES_IN_AN_HOUR;
            valid_hours += minus_hours;
            if minus_hours != 0 {
                valid_minutes = MINUTES_IN_AN_HOUR + valid_minutes % (minus_hours*MINUTES_IN_AN_HOUR);
            } else {
                println!("minus_hours: {}-{}", minus_hours, MINUTES_IN_AN_HOUR + valid_minutes);
                valid_minutes = MINUTES_IN_AN_HOUR + valid_minutes;
                valid_hours -= 1;
            }
            println!("calculate: {}:{}", valid_hours, valid_minutes);
        }
        while valid_hours < 0 {
            valid_hours = HOURS_IN_A_DAY + valid_hours;
        }
        while valid_minutes >= MINUTES_IN_AN_HOUR {
            valid_hours += valid_minutes/MINUTES_IN_AN_HOUR;
            valid_minutes = valid_minutes % MINUTES_IN_AN_HOUR;
        }
        while valid_hours >= HOURS_IN_A_DAY {
            valid_hours = valid_hours % HOURS_IN_A_DAY;
        }
        println!("{}:{} vs {}:{}", hours, minutes, valid_hours, valid_minutes);
        Clock{ hours: valid_hours, minutes: valid_minutes}
    }

    pub fn add_minutes(self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        println!("result: {:02}:{:02}", &self.hours, &self.minutes);
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

