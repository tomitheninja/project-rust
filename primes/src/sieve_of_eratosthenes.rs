//! Find prime numbers using sieve of eratosthenes algorithm
//!
//! It finds all primes in range `1..n`

/// returns an array containing `size` elements
///
/// element at index `i` with true value indicates that __`i` is a prime__ number
/// # Examples
/// ```rust
/// let is_prime = primes::sieve_of_eratosthenes::meta(5);
///
/// assert_eq!(true, is_prime[2] && is_prime[3]);
/// assert_eq!(false, is_prime[0] || is_prime[1] || is_prime[4]);
/// ```
pub fn meta(size: usize) -> Vec<bool> {
    let n = size;
    assert!(n > 1);
    let mut is_prime = vec![true; n];
    is_prime[0] = false;
    is_prime[1] = false;

    let sqrt_n = (n as f64).sqrt().ceil() as usize;

    for i in 2..=sqrt_n {
        if !is_prime[i] {
            // Optimization:
            // multiples are already false
            // nothing to do
            continue;
        }
        // at this point we are sure that i is a prime
        for multiplies in (i..n).step_by(i).skip(1) {
            // mark all of i's multiples as complex numbers
            is_prime[multiplies] = false;
        }
    }
    is_prime
}

/// returns a vector containing all primes below n
/// # Examples
/// ```rust
/// use primes::sieve_of_eratosthenes;
///
/// assert_eq!(vec![2, 3, 5], sieve_of_eratosthenes::collected(6));
/// ```
pub fn collected(n: usize) -> Vec<usize> {
    self::meta(n)
        .into_iter()
        .zip(0..)
        .filter(|(is_prime, _)| *is_prime)
        .map(|(_, i)| i)
        .collect()
}

#[cfg(test)]
mod prime_filter {

    #[test]
    fn asd() {
        let result = super::collected(10);
        assert_eq!(vec![2, 3, 5, 7], result);
    }

    #[test]
    fn primes_below_10() {
        let is_prime = super::meta(10);
        assert!(!is_prime[0]);
        assert!(!is_prime[1]);
        assert!(is_prime[2]);
        assert!(is_prime[3]);
        assert!(!is_prime[4]);
        assert!(is_prime[5]);
        assert!(!is_prime[6]);
        assert!(is_prime[7]);
        assert!(!is_prime[8]);
        assert!(!is_prime[9]);
    }
}
