//! By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
//!
//! What is the 10 001st prime number?

extern crate primes;

fn main() {
    println!("{}", solve(10_001));
}

fn get_vector_size(n: usize) -> usize {
    let mut size = 16;
    loop {
        if (n as f64) < primes::many_primes_below(size) {
            return (size as f64 * 1.25) as _;
        }
        size *= 2;
    }
}

fn solve(n: usize) -> usize {
    assert!(n != 0);
    let is_prime = primes::sieve_of_eratosthenes(get_vector_size(n));
    is_prime
        .iter()
        .enumerate()
        .filter(|(_, &val)| val)
        .map(|(i, _)| i)
        .nth(n - 1)
        .unwrap()
}

#[cfg(test)]
mod p007 {
    use super::solve;

    #[test]
    fn sixth() {
        assert_eq!(13, solve(6));
    }

    #[test]
    fn ten_thousand_one_th() {
        assert_eq!(104743, solve(10_001));
    }
}
