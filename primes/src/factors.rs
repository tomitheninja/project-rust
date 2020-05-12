//! Prime factors are a set of prime numbers. The product of this set is the original number
//! A prime factor can present multiple times

/// Returns all prime factors for `n`
///
/// The prime factors are in ascending order
///
/// Because one is not a prime number, it is skipped
/// # Examples
/// ```rust
/// use primes::prime_factors;
///
/// assert_eq!(vec![11], prime_factors(11));
/// assert_eq!(vec![2, 2, 3], prime_factors(12));
/// ```
pub fn prime_factors(n: u64) -> Vec<u64> {
    PrimeFactors::new(n).collect()
}

struct PrimeFactors {
    remaining_value: u64,
}

impl PrimeFactors {
    pub fn new(value: u64) -> Self {
        PrimeFactors {
            remaining_value: value,
        }
    }
}

impl Iterator for PrimeFactors {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let result = (2..=self.remaining_value).find(|&x| self.remaining_value % x == 0);
        if let Some(factor) = result {
            self.remaining_value /= factor;
        }
        result
    }
}

#[cfg(test)]
mod factors {
    use super::prime_factors;

    #[test]
    fn primes_have_one_factor() {
        assert_eq!(vec![11], prime_factors(11))
    }

    #[test]
    fn a_num_equals_to_the_product_of_its_factors() {
        assert_eq!(123456_u64, prime_factors(123456).into_iter().product());
    }

    #[test]
    fn primes_are_in_asc_order() {
        let iter: Vec<u64> = prime_factors(1234);
        let mut sorted_iter = iter.clone();
        sorted_iter.sort();
        assert_eq!(iter, sorted_iter);
    }
}
