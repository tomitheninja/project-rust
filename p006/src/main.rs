//! The sum of the squares of the first ten natural numbers is,
//!  1<sup>2</sup> + 2<sup>2</sup> + ... + 10<sup>2</sup> = 385
//!
//! The square of the sum of the first ten natural numbers is,
//! (1+2+...+10)<sup>2</sup> = 55<sup>2</sup> = 3025
//!
//! Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is 3025−385=2640
//!
//! Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.

fn main() {
    println!("{}", solve(100));
}

fn solve(n: u64) -> u64 {
    let res1 = sum_of_integers_below(n);
    let res2 = sum_of_integers_squared_below(n);
    res1 * res1 - res2
}

fn sum_of_integers_below(n: u64) -> u64 {
    (1 + n) * n / 2
}

fn sum_of_integers_squared_below(n: u64) -> u64 {
    n * (n + 1) * (2 * n + 1) / 6
}

#[cfg(test)]
mod p006 {
    use super::{solve, sum_of_integers_below, sum_of_integers_squared_below};

    #[test]
    fn sum_of_integers() {
        assert_eq!(55, sum_of_integers_below(10));
    }

    #[test]
    fn sum_of_integers_squared() {
        assert_eq!(385, sum_of_integers_squared_below(10));
    }

    #[test]
    fn solve_for_10() {
        assert_eq!(2640, solve(10));
    }

    #[test]
    fn solve_for_100() {
        assert_eq!(25164150, solve(100))
    }
}
