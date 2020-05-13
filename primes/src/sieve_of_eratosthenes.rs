/// Find all prime numbers below n
pub fn sieve_of_eratosthenes(n: usize) -> Vec<bool> {
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

#[cfg(test)]
mod prime_filter {
    use super::sieve_of_eratosthenes;

    #[test]
    fn primes_below_10() {
        let is_prime = sieve_of_eratosthenes(10);
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
