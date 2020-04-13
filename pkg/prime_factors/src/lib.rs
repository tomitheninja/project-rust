pub fn get_smallest_prime_factor(n: u64) -> Option<u64> {
    match n {
        0 | 1 => None,
        _ => (2..=n).filter(|x| n % x == 0).nth(0),
    }
}

pub struct PrimeFactor(u64);

impl PrimeFactor {
    pub fn new(value: u64) -> Self {
        PrimeFactor(value)
    }
}

impl Iterator for PrimeFactor {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            return None;
        }
        let maybe_factor = get_smallest_prime_factor(self.0);
        if let Some(div) = maybe_factor { self.0 /= div }
        maybe_factor
    }
}

#[cfg(test)]
mod test_smallest_prime_factor {
    use super::{get_smallest_prime_factor, PrimeFactor};

    #[test]
    fn it_should_not_work_with_zero() {
        assert_eq!(None, get_smallest_prime_factor(0));
    }
    #[test]
    fn it_should_not_work_with_one() {
        assert_eq!(None, get_smallest_prime_factor(1));
    }
    #[test]
    fn it_should_work_with_two() {
        assert_eq!(Some(2), get_smallest_prime_factor(2));
    }
    #[test]
    fn it_should_work_with_four() {
        assert_eq!(Some(2), get_smallest_prime_factor(4));
    }

    #[test]
    fn a_number_equals_the_product_of_its_prime_factors() {
        const RANDOM_INT: u64 = 5040;
        assert_eq!(RANDOM_INT, PrimeFactor::new(RANDOM_INT).product());
    }
}
