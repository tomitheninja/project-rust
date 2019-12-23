#![allow(dead_code)]

pub mod sequence {
    // Fibonacci
    pub struct Fibonacci {
        n1: u32,
        n2: u32,
    }

    impl Fibonacci {
        pub fn new() -> Fibonacci {
            Fibonacci { n1: 0, n2: 1 }
        }
    }

    impl Iterator for Fibonacci {
        type Item = u32;
        fn next(&mut self) -> Option<Self::Item> {
            let sum = self.n1 + self.n2;
            self.n1 = self.n2;
            self.n2 = sum;
            Some(self.n1)
        }
    }

    // Triangle number
    /// Represents a number that calculated like this: 1+2+3+4...+n
    pub struct TriangleNumber {
        value: u64,
        sum: u64,
    }

    impl TriangleNumber {
        pub fn new() -> TriangleNumber {
            TriangleNumber { value: 1, sum: 1 }
        }
    }

    impl Iterator for TriangleNumber {
        type Item = u64;
        fn next(&mut self) -> Option<Self::Item> {
            self.value += 1;
            self.sum += self.value;
            Some(self.sum)
        }
    }

    // CollatzSequence
    // Represents a sequence that goes to 1
    pub struct CollatzSequence {
        value: u64,
    }

    impl CollatzSequence {
        pub fn new(starting_value: u64) -> CollatzSequence {
            CollatzSequence {
                value: starting_value,
            }
        }
    }

    fn next_collatz_item(n: u64) -> Option<u64> {
        if n < 2 {
            None
        } else if n % 2 == 0 {
            Some(n / 2)
        } else {
            Some(3 * n + 1)
        }
    }

    #[test]
    fn test_next_collatz_item() {
        assert_eq!(next_collatz_item(13).unwrap(), 40);
        assert_eq!(next_collatz_item(20).unwrap(), 10);
        assert_eq!(next_collatz_item(2).unwrap(), 1);
        match next_collatz_item(1) {
            Some(_x) => panic!("Should return None"),
            None => (),
        }
    }

    impl Iterator for CollatzSequence {
        type Item = u64;
        fn next(&mut self) -> Option<Self::Item> {
            match next_collatz_item(self.value) {
                None => None,
                Some(next_value) => {
                    self.value = next_value;
                    Some(next_value)
                }
            }
        }
    }
}

pub mod date {
    #[derive(PartialEq, Eq, Copy, Clone, Debug)]
    pub enum Month {
        January = 1,
        February,
        March,
        April,
        May,
        June,
        July,
        August,
        September,
        October,
        November,
        December,
    }

    #[derive(PartialEq, Eq, Copy, Clone, Debug)]
    pub enum DayOfWeek {
        Monday = 1,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday,
    }

    #[derive(Copy, Clone, Debug)]
    pub struct Date {
        year: u16,
        month: Month,
        day: u8,
        day_of_week: DayOfWeek,
    }

    impl Month {
        pub fn next(&self) -> Month {
            match self {
                Month::January => Month::February,
                Month::February => Month::March,
                Month::March => Month::April,
                Month::April => Month::May,
                Month::May => Month::June,
                Month::June => Month::July,
                Month::July => Month::August,
                Month::August => Month::September,
                Month::September => Month::October,
                Month::October => Month::November,
                Month::November => Month::December,
                Month::December => Month::January,
            }
        }
    }

    impl DayOfWeek {
        pub fn next(&self) -> DayOfWeek {
            match self {
                DayOfWeek::Monday => DayOfWeek::Tuesday,
                DayOfWeek::Tuesday => DayOfWeek::Wednesday,
                DayOfWeek::Wednesday => DayOfWeek::Thursday,
                DayOfWeek::Thursday => DayOfWeek::Friday,
                DayOfWeek::Friday => DayOfWeek::Saturday,
                DayOfWeek::Saturday => DayOfWeek::Sunday,
                DayOfWeek::Sunday => DayOfWeek::Monday,
            }
        }
    }

    impl Date {
        pub fn new() -> Date {
            Date {
                year: 1900,
                month: Month::January,
                day: 1,
                day_of_week: DayOfWeek::Monday,
            }
        }

        pub fn is_leap_year(&self) -> bool {
            let year = self.year;
            year % 4 == 0 && year % 100 != 0 || year % 400 == 0
        }

        pub fn days_in_month(&self) -> usize {
            match self.month {
                Month::September | Month::April | Month::June | Month::November => 30,
                Month::February => {
                    if self.is_leap_year() {
                        29
                    } else {
                        28
                    }
                }
                _ => 31,
            }
        }

        pub fn get_year(&self) -> u16 {
            self.year
        }

        pub fn get_month(&self) -> Month {
            self.month
        }

        pub fn get_day(&self) -> u8 {
            self.day
        }

        pub fn get_day_of_week(&self) -> DayOfWeek {
            self.day_of_week
        }

        pub fn get_tomorrow_date(&self) -> Date {
            let month_change = self.day + 1 > self.days_in_month() as u8;
            let year_change = month_change && self.month == Month::December;

            let tomorrow_day = if month_change { 1 } else { self.day + 1 };
            let tomorrow_month = if month_change {
                self.month.next()
            } else {
                self.month
            };
            let tomorrow_year = if year_change {
                self.year + 1
            } else {
                self.year
            };
            let tomorrow_day_of_week = self.day_of_week.next();

            Date {
                day: tomorrow_day,
                month: tomorrow_month,
                year: tomorrow_year,
                day_of_week: tomorrow_day_of_week,
            }
        }
    }

    impl Iterator for Date {
        type Item = Date;
        fn next(&mut self) -> Option<Self::Item> {
            let tomorrow = self.get_tomorrow_date();
            self.day_of_week = tomorrow.day_of_week;
            self.day = tomorrow.day;
            self.month = tomorrow.month;
            self.year = tomorrow.year;

            Some(tomorrow)
        }
    }
}
