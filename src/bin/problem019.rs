use std::ops::{Add, AddAssign};

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq)]
enum Weekday {
    Monday = 0,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl From<Weekday> for u8 {
    fn from(value: Weekday) -> Self {
        value as u8
    }
}

impl Add<u8> for Weekday {
    type Output = Weekday;

    fn add(self, days: u8) -> Self::Output {
        unsafe { std::mem::transmute((self as u8 + days % 7) % 7) }
    }
}

impl Add<Weekday> for u8 {
    type Output = Weekday;

    fn add(self, weekday: Weekday) -> Self::Output {
        weekday + self
    }
}

impl AddAssign<u8> for Weekday {
    fn add_assign(&mut self, days: u8) {
        *self = *self + days;
    }
}

struct Period {
    year: u16,
    month: u8,
}

impl Period {
    pub fn new(year: u16, month: u8) -> Self {
        Self { year, month }
    }

    pub fn is_leap_year(&self) -> bool {
        self.year % 400 == 0 || (self.year % 4 == 0 && self.year % 100 != 0)
    }

    pub fn month_length_in_days(&self) -> u8 {
        match self.month {
            2 => {
                if self.is_leap_year() {
                    29
                } else {
                    28
                }
            }
            4 | 6 | 9 | 11 => 30,
            _ => 31,
        }
    }
}

fn main() {
    let mut current_weekday = Weekday::Monday;
    for month in 1..=12 {
        current_weekday += Period::new(1900, month).month_length_in_days();
    }
    let mut sundays = 0;
    for year in 1901..=2000 {
        for month in 1..=12 {
            if year == 2000 && month == 12 {
                break;
            }

            current_weekday += Period::new(year, month).month_length_in_days();
            if current_weekday == Weekday::Sunday {
                sundays += 1;
            }
        }
    }
    println!("Problem 019: {}", sundays);
}
