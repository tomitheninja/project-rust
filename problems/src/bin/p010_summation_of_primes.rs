// Summation of primes

// The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.

// Find the sum of all the primes below two million.

extern crate find_primes;

use find_primes::find_primes_below;

fn compute(bound: usize) -> u64 {
    let is_prime = find_primes_below(bound);

    (0..bound).filter(|i| is_prime[*i]).sum::<usize>() as u64
}

fn main() {
    println!("p010: {}", compute(2_000_000));
}

#[cfg(test)]
mod p010 {
    use super::*;

    #[test]
    fn below_10() {
        assert_eq!(17, compute(10));
    }

    #[test]
    fn below_two_million() {
        assert_eq!(922, compute(2_000_000) % 1_000);
    }
}
