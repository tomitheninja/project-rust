// Summation of primes

// The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.

// Find the sum of all the primes below two million.

fn get_prime_table(n: usize) -> Vec<bool> {
    assert!(n > 2);
    let mut v = vec![true; n];
    v[0] = false;
    v[1] = false;

    for step in 2..n {
        if !v[step] {
            continue;
        } // Already false
        for factor in (2 * step..n).step_by(step) {
            v[factor] = false;
        }
    }
    v
}

fn main () {
    let is_prime = get_prime_table(2_000_000);

    let mut sum: usize = 0;
    for prime in 0..is_prime.len() {
        if is_prime[prime] {
            sum += prime;
        }
    }

    println!("{}", sum);
}
