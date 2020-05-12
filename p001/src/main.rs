//! If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9.
//! The sum of these multiples is 23.
//!
//! Find the sum of all the multiples of 3 or 5 below 1000.

fn main() {
    println!("{}", solve(1000));
}

/// Sum all numbers below bound that are multiplies of 3 or 5
fn solve(bound: usize) -> u64 {
    (1..bound as u64)
        .filter(|&x| x % 3 == 0 || x % 5 == 0)
        .sum()
}

#[cfg(test)]
mod p001 {
    use super::solve;

    #[test]
    fn below_10() {
        assert_eq!(23, solve(10));
    }

    #[test]
    fn below_1000() {
        assert_eq!(233168, solve(1000));
    }
}
