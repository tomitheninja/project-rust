/// Finds all primes in the range 0..n
///
/// Algorithm: Sieve of Eratosthenes
pub fn find_primes_below(n: usize) -> Vec<bool> {
    assert!(n >= 2);
    let mut is_prime = vec![true; n];
    is_prime[0] = false;
    is_prime[1] = false;

    let n_sqrt = (n as f64).sqrt() as usize;

    for i in 2..=n_sqrt as usize {
        // At this point we already know, if i is prime or not
        if !is_prime[i] {
            // optimization: do not mark it's multiplies again
            continue;
        }
        // i is prime
        // mark all of it's multiplies as complex numbers
        for j in (i..n).step_by(i).skip(1) {
            is_prime[j] = false;
        }
    }
    is_prime
}

#[cfg(test)]
mod is_prime {
    use super::find_primes_below;

    #[test]
    fn zero_and_one_are_not_primes() {
        let x = find_primes_below(2);
        assert_eq!(x[0], false);
        assert_eq!(x[1], false);
    }

    #[test]
    fn two_is_prime() {
        let x = find_primes_below(3);
        assert_eq!(x[2], true);
    }

    #[test]
    fn four_is_complex() {
        let x = find_primes_below(5);
        assert_eq!(x[4], false);
    }

    #[test]
    fn five_is_prime() {
        let x = find_primes_below(6);
        assert_eq!(true, x[5]);
    }
}
