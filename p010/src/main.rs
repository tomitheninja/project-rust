//! The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
//!
//! Find the sum of all the primes below two million.

extern crate primes;

fn main() {
    println!("{}", solve(2_000_000));
}

/// Sum of primes below `n`
fn solve(n: usize) -> u64 {
    primes::sieve_of_eratosthenes::collected(n)
        .into_iter()
        .map(|x| x as u64)
        .sum()
}

#[cfg(test)]
mod p010 {
    use super::solve;

    #[test]
    fn solve_for_10() {
        assert_eq!(17, solve(10));
    }

    #[test]
    fn solve_for_two_million() {
        assert_eq!(142913828922, solve(2_000_000))
    }
}
