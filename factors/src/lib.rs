extern crate primes;

use primes::prime_factors;

pub struct Factorable {
    value: u64,
}
impl Factorable {
    pub fn new(value: u64) -> Self {
        assert!(value != 0);
        Factorable { value }
    }

    /// returns a vector that contains all factors of `n`
    /// # Examples
    /// ```rust
    /// use factors::Factorable;
    ///
    /// assert_eq!(vec![1, 2, 4, 8, 16], Factorable::new(16).factors());
    /// ```
    pub fn factors(&self) -> Vec<u64> {
        let n = self.value;
        let sqrt = (n as f64).sqrt() as u64;
        let mut result = Vec::new();
        for factor1 in (1..=sqrt).filter(|&maybe_factor| n % maybe_factor == 0) {
            let factor2 = n / factor1;
            result.push(factor1);
            if factor1 != factor2 {
                result.push(factor2);
            }
        }
        result.sort();
        result
    }

    /// Like `self::factors`, but only returns the amount of the factors
    ///
    /// It should be faster
    /// # Examples
    /// ```rust
    /// use factors::Factorable;
    ///
    /// assert_eq!(5, Factorable::new(16).num_factors());
    /// ```
    pub fn num_factors(&self) -> usize {
        let n = self.value;
        let sqrt = (n as f64).sqrt() as u64;
        let mut many = 0;
        for factor in (1..=sqrt).filter(|&maybe_factor| n % maybe_factor == 0) {
            let is_sqrt_factor = factor == n / factor;
            many += if is_sqrt_factor { 1 } else { 2 }
        }
        many
    }

    /// Returns all prime factors for `n`
    ///
    /// The prime factors are in ascending order
    ///
    /// Because one is not a prime number, it is skipped
    /// # Examples
    /// ```rust
    /// use factors::Factorable;
    ///
    /// assert_eq!(vec![11], Factorable::new(11).prime_factors());
    /// assert_eq!(vec![2, 2, 3], Factorable::new(12).prime_factors());
    /// ```
    pub fn prime_factors(&self) -> Vec<u64> {
        prime_factors(self.value)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn prime_factors_of_12() {
        assert_eq!(vec![2, 2, 3], super::prime_factors(12));
    }
}
