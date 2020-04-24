// I should have used int-s...

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
    pub fn next(self) -> Self {
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
    pub fn next(self) -> Self {
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
    pub fn is_leap_year(self) -> bool {
        let year = self.year;
        year % 4 == 0 && year % 100 != 0 || year % 400 == 0
    }

    pub fn days_in_month(self) -> usize {
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

    pub fn get_year(self) -> u16 {
        self.year
    }

    pub fn get_month(self) -> Month {
        self.month
    }

    pub fn get_day(self) -> u8 {
        self.day
    }

    pub fn get_day_of_week(self) -> DayOfWeek {
        self.day_of_week
    }

    pub fn get_tomorrow_date(self) -> Self {
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

impl Default for Date {
    fn default() -> Self {
        Date {
            year: 1900,
            month: Month::January,
            day: 1,
            day_of_week: DayOfWeek::Monday,
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
