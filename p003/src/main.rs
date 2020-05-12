//! The prime factors of 13195 are 5, 7, 13 and 29.
//!
//! What is the largest prime factor of the number 600851475143 ?

extern crate primes;

fn main() {
    println!("{}", solve(600851475143));
}

/// Returns the largest prime factor for a number
fn solve(n: u64) -> u64 {
    primes::prime_factors(n).into_iter().last().unwrap()
}

#[cfg(test)]
mod p003 {
    use super::solve;

    #[test]
    fn largest_factor_of_13195() {
        assert_eq!(29, solve(13195));
    }

    #[test]
    fn largest_factor_of_600851475143() {
        assert_eq!(6857, solve(600851475143));
    }
}
