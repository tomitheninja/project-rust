// 10001st prime

// By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.

// What is the 10 001st prime number?

fn get_prime_table(n: usize) -> Vec<bool> {
    assert!(n > 2);
    let mut v = vec![true; n];
    v[0] = false;
    v[1] = false;

    for step in 2..n {
        if !v[step] {
            continue;  // Skip if already false
        }
        for factor in (2 * step..n).step_by(step) {
            v[factor] = false;
        }
    }
    v
}

#[allow(dead_code)]
pub fn run(n_th: usize) -> usize {
    let is_prime = get_prime_table(999999);

    (0..is_prime.len())
        .filter(|i| is_prime[*i])
        .nth(n_th - 1).unwrap()
}

#[test]
fn test () {
    assert_eq!(104743, run(10_001));
}
