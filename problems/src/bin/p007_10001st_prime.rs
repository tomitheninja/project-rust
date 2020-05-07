// 10001st prime

// By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.

// What is the 10 001st prime number?
extern crate find_primes;

use find_primes::find_primes_below;

fn compute(n_th: usize) -> usize {
    // 2^31 has 20.4% primes
    // n * 25 should work in the u32 range
    let is_prime = find_primes_below(n_th * 25);

    (0..is_prime.len())
        .filter(|i| is_prime[*i])
        .nth(n_th - 1)
        .unwrap()
}

fn main() {
    println!("p007: {}", compute(10_001));
}

#[cfg(test)]
mod p007 {
    use super::*;

    #[test]
    fn test_for_6() {
        assert_eq!(13, compute(6));
    }

    #[test]
    fn test_for_10001() {
        assert_eq!(43, compute(10_001) % 100);
    }
}
