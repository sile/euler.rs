//! [19] Counting Sundays
//! -----------------------
//!
pub fn solve() -> u64 {
    Date::new()
        .skip_while(|d| d.to_tuple() < (1901, 1, 1))
        .take_while(|d| d.to_tuple() <= (2000, 12, 31))
        .filter(|d| d.weekday == 6)
        .filter(|d| d.day == 0)
        .count() as u64
}

#[derive(Clone)]
struct Date {
    year: usize,
    month: usize, // 0~11
    day: usize, // 0~30
    weekday: usize, // 0~6 (0=monday)
}

impl Date {
    pub fn new() -> Date {
        Date {
            year: 1900,
            month: 0,
            day: 0,
            weekday: 0,
        }
    }
    pub fn to_tuple(&self) -> (usize, usize, usize) {
        (self.year, self.month + 1, self.day + 1)
    }
    pub fn increment_day(&mut self) {
        self.weekday = (self.weekday + 1) % 7;
        self.day = (self.day + 1) % self.month_days();
        if self.day == 0 {
            self.month = (self.month + 1) % 12;
            if self.month == 0 {
                self.year += 1;
            }
        }
    }
    fn month_days(&self) -> usize {
        match self.month {
            0 => 31,
            1 => if self.is_leap_year() { 29 } else { 28 },
            2 => 31,
            3 => 30,
            4 => 31,
            5 => 30,
            6 => 31,
            7 => 31,
            8 => 30,
            9 => 31,
            10 => 30,
            11 => 31,
            _ => unreachable!(),
        }
    }
    fn is_leap_year(&self) -> bool {
        self.year % 4 == 0 && (self.year % 100 != 0 || self.year % 400 == 0)
    }
}

impl Iterator for Date {
    type Item = Date;
    fn next(&mut self) -> Option<Date> {
        let cur = self.clone();
        self.increment_day();
        Some(cur)
    }
}
