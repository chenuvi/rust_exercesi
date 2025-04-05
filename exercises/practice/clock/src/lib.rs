pub struct Clock;

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            hours: hours,
            minutes: minutes,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let mut new_hours = self.hours;
        let mut new_minutes = self.minutes + minutes;
        while new_minutes < 0 {
            new_minutes += 60;
            new_hours -= 1;
        }
        while new_minutes >= 60 {
            new_minutes -= 60;
            new_hours += 1; 
        }
    }
}
