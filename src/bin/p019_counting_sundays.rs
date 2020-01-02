// Counting Sundays

// You are given the following information, but you may prefer to do some research for yourself.

// - 1 Jan 1900 was a Monday.
// - Thirty days has September,
//   April, June and November.
//   All the rest have thirty-one,
//   Saving February alone,
//   Which has twenty-eight, rain or shine.
//   And on leap years, twenty-nine.
// - A leap year occurs on any year evenly divisible by 4, but not on a century unless it is divisible by 400.

// How many Sundays fell on the first of the month during the twentieth century (1 Jan 1901 to 31 Dec 2000)?

extern crate date;
use date::{Date, DayOfWeek};

fn compute(first_year: u16, last_year: u16, date: u8, day_of_week: DayOfWeek) -> usize {
    assert!(first_year >= 1900);
    Date::default()
        .skip_while(|d| d.get_year() <= first_year)
        .take_while(|d| d.get_year() <= last_year)
        .filter(|d| d.get_day() == date)
        .filter(|d| d.get_day_of_week() == day_of_week)
        .count()
}

fn main () {
    println!("p019: {}", compute(1900, 2000, 1, DayOfWeek::Sunday));
}


#[cfg(test)]
mod test_p019 {
    use super::*;
    use date::DayOfWeek;

    #[test]
    fn test_20th_century() {
        assert_eq!(71, compute(1900, 2000, 1, DayOfWeek::Sunday) % 100);
    }
}
