// Largest prime factor

// The prime factors of 13195 are 5, 7, 13 and 29.

// What is the largest prime factor of the number 600851475143 ?

extern crate prime_factors;
use prime_factors::PrimeFactor;

fn compute(n: u64) -> u64 {
    PrimeFactor::new(n).last().unwrap()
}

fn main() {
    println!("p003: {}", compute(600_851_475_143));
}

#[cfg(test)]
mod p003_tests {
    use super::compute;

    #[test]
    fn test_13195() {
        assert_eq!(29, compute(13195));
    }

    #[test]
    fn test_600851475143() {
        assert_eq!(57, compute(600_851_475_143) % 100);
    }
}
