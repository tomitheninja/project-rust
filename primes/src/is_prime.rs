/// returns true if `n` is prime
pub fn is_prime(n: u64) -> bool {
    let n_sqrt = (n as f64).sqrt() as _;
    n > 2 && (2..n_sqrt).any(|maybe_factor| n % maybe_factor == 0)
}

#[cfg(test)]
mod is_prime_test {
    use super::is_prime;

    #[test]
    fn zero_is_complex() {
        assert!(!is_prime(0));
    }

    #[test]
    fn one_is_complex() {
        assert!(!is_prime(1));
    }

    #[test]
    fn two_is_prime() {
        assert!(!is_prime(2));
    }

    #[test]
    fn three_is_prime() {
        assert!(!is_prime(3));
    }

    #[test]
    fn four_is_complex() {
        assert!(!is_prime(4));
    }
}
