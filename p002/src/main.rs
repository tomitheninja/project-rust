//! Each new term in the Fibonacci sequence is generated by adding the previous two terms.
//! By starting with 1 and 2, the first 10 terms will be:
//! 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...
//! By considering the terms in the Fibonacci sequence whose values do not exceed four million,
//! find the sum of the even-valued terms.

extern crate fibonacci_sequence;

use fibonacci_sequence::FibonacciSequence;

fn main() {
    println!("{}", solve(4_000_000));
}

/// Sum of even fibonacci numbers below `bound`
fn solve(bound: usize) -> u64 {
    FibonacciSequence::new()
        .take_while(|&x| x < bound as u64)
        .filter(|&x| x % 2 == 0)
        .sum()
}

#[cfg(test)]
mod p002 {
    use super::solve;

    #[test]
    fn below_10() {
        let result = solve(10);
        assert_eq!(2 + 8, result);
    }

    #[test]
    fn below_four_million() {
        let result = solve(4_000_000);
        assert_eq!(4613732, result);
    }
}
