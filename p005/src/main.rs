//! 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
//!
//! What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?

fn main() {
    println!("{}", solve(20));
}

/// Greatest common divisor
fn gcd(a: u64, b: u64) -> u64 {
    match a {
        0 => b,
        _ => gcd(b % a, a),
    }
}

/// Least common multiple
fn lcm(a: u64, b: u64) -> u64 {
    let product = a * b;
    product / gcd(a, b)
}

/// least common multiple of all integers in range 1..`n`
fn solve(n: usize) -> u64 {
    (1..n).fold(1, |acc, x| lcm(acc, x as _))
}

#[cfg(test)]
mod p005 {
    use super::solve;

    #[test]
    fn below_10() {
        assert_eq!(2520, solve(10));
    }

    #[test]
    fn below_20() {
        assert_eq!(232792560, solve(20));
    }
}
