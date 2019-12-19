// Number letter counts
// If the numbers 1 to 5 are written out in words: one, two, three, four, five, then there are 3 + 3 + 5 + 4 + 4 = 19 letters used in total.

// If all the numbers from 1 to 1000 (one thousand) inclusive were written out in words, how many letters would be used?
// NOTE: Do not count spaces or hyphens. For example, 342 (three hundred and forty-two) contains 23 letters and 115 (one hundred and fifteen) contains 20 letters. The use of "and" when writing out numbers is in compliance with British usage.

const ONE: usize = 3; // 1
const TWO: usize = 3; // 2
const THREE: usize = 5; // 3
const FOUR: usize = 4; // 4
const FIVE: usize = 4; // 5
const SIX: usize = 3; // 6
const SEVEN: usize = 5; // 7
const EIGHT: usize = 5; // 7
const NINE: usize = 4; // 9

const TEN: usize = 3; // 10
const ELEVEN: usize = 6; // 11
const TWELVE: usize = 6; // 12
const THIRTEEN: usize = 8; // 13
const FIFTEEN: usize = 7; // 15
const EIGHTEEN: usize = 8; // 18

const TEEN: usize = 4; // 10
const TWENTY: usize = 6; // 20
const THIRTY: usize = 6; // 30
const FORTY: usize = 5; // 40
const FIFTY: usize = 5; // 50
const SIXTY: usize = 5; // 60
const SEVENTY: usize = 7; // 70
const EIGHTY: usize = 6; // 80
const NINETY: usize = 6; // 90

const HUNDRED: usize = 7;
const HUNDRED_AND: usize = HUNDRED + 3; // x*100 and
const THOUSAND: usize = 8; // 1000


/// DOES NOT WORK WITH 0
fn length(n: usize) -> usize {
    match n {
        1 => ONE,
        2 => TWO,
        3 => THREE,
        4 => FOUR,
        5 => FIVE,
        6 => SIX,
        7 => SEVEN,
        8 => EIGHT,
        9 => NINE,
        10 => TEN,
        11 => ELEVEN,
        12 => TWELVE,
        13 => THIRTEEN,
        15 => FIFTEEN,
        18 => EIGHTEEN,
        (10..=19) => TEEN + length(n % 10),
        (20..=29) => TWENTY + length(n % 10),
        (30..=39) => THIRTY + length(n % 10),
        (40..=49) => FORTY + length(n % 10),
        (50..=59) => FIFTY + length(n % 10),
        (60..=69) => SIXTY + length(n % 10),
        (70..=79) => SEVENTY + length(n % 10),
        (80..=89) => EIGHTY + length(n % 10),
        (90..=99) => NINETY + length(n % 10),
        100 | 200 | 300 | 400 | 500 | 600 | 700 | 800 | 900 => length(n / 100) + HUNDRED,
        (100..=999) => length(n / 100) + HUNDRED_AND + length(n % 100),
        (1000..=1999) => length(n / 1000) + THOUSAND + length(n % 100),
        _ => 0,
    }
}

fn main () {
    
    let result = (1..=1000)
        .map(|i| length(i))
        .sum::<usize>();
    println!("{}", result);
}
