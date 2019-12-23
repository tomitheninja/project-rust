// Largest prime factor

// The prime factors of 13195 are 5, 7, 13 and 29.

// What is the largest prime factor of the number 600851475143 ?

fn get_smallest_factor(n: u64) -> u64 {
    match n {
        1 => 1,
        _ => (2..).filter(|&x| n % x == 0).nth(0).unwrap(),
    }
}

fn get_largest_prime_factor(n: u64) -> u64 {
    let mut remaining = n;
    let mut latest_factor = 1;
    // smallest_factor()'s last output is the largest one
    while remaining > 1 {
        latest_factor = get_smallest_factor(remaining);
        remaining /= latest_factor;
    }
    latest_factor
}

#[allow(dead_code)]
pub fn run(n: u64) -> u64 {
    get_largest_prime_factor(n)
}

#[test]
fn test() {
    assert_eq!(run(600851475143), 6857);
}
