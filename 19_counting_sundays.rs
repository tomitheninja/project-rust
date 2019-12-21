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

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum Month { 
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
enum DayOfWeek {
    Monday = 1,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

#[derive(Copy, Clone, Debug)]
struct Date {
    year: u16,
    month: Month,
    day: u8,
    day_of_week: DayOfWeek,
}

impl Month {

    fn next (&self) -> Month {
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
    fn next (&self) -> DayOfWeek {
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

    fn new () -> Date {
        Date {
            year: 1900,
            month: Month::January,
            day: 1,
            day_of_week: DayOfWeek::Monday,
        }
    }

    fn is_leap_year (&self) -> bool {
        let year = self.year;
        year % 4 == 0 && year % 100 != 0 || year % 400 == 0
    }

    fn days_in_month (&self) -> usize {
        match self.month {
            Month::September | Month::April | Month::June | Month::November => 30,
            Month::February => if self.is_leap_year() { 29 } else { 28 },            
            _ => 31,
        }
    }

    fn get_tomorrow_date (&self) -> Date {
        
        let month_change = self.day + 1 > self.days_in_month() as u8;
        let year_change = month_change && self.month == Month::December;

        let tomorrow_day = if month_change { 1 } else { self.day + 1 };
        let tomorrow_month = if month_change { self.month.next() } else { self.month };
        let tomorrow_year = if year_change { self.year + 1 } else { self.year };
        let tomorrow_day_of_week = self.day_of_week.next();

        Date {
            day: tomorrow_day,
            month: tomorrow_month,
            year: tomorrow_year,
            day_of_week: tomorrow_day_of_week,
        }
    }

    // fn to_string (&self) -> String {
    //     let mut s = String::new();
    //     s.push_str(&self.year.to_string());
    //     s.push(' ');
    //     s.push_str(self.month.to_string());
    //     s.push(' ');
    //     s.push_str(&self.day.to_string());
    //     s
    // }
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

fn main () {
    let it = Date::new()
        .skip_while(|d| d.year < 1901)
        .take_while(|d| d.year < 2001)
        .filter(|d| d.day == 1)
        .filter(|d| d.day_of_week == DayOfWeek::Sunday);

    println!("{}", it.count());
}
