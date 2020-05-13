//! A palindromic number reads the same both ways.
//! The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
//!
//! Find the largest palindrome made from the product of two 3-digit numbers.

extern crate is_palindrome;

fn main() {
    println!("{}", solve(3).unwrap());
}

fn solve(num_digits: usize) -> Option<u64> {
    assert!(num_digits > 0);
    let start = 10.0_f64.powi((num_digits - 1) as _).round() as usize;
    let end = start * 10;

    let mut best = 0;

    for x1 in (start..end).rev() {
        for x2 in (start..end).rev() {
            let product = (x1 * x2) as _;
            if product <= best {
                // optimization: `x2` is decreasing, it will never be better than best
                break;
            }
            if is_palindrome::num(product) {
                best = product;
            }
        }
    }
    match best {
        0 => None,
        _ => Some(best),
    }
}

#[cfg(test)]
mod p004 {
    use super::solve;

    #[test]
    fn two_digits() {
        assert_eq!(Some(9009), solve(2));
    }

    #[test]
    fn three_digits() {
        assert_eq!(Some(906609), solve(3));
    }
}
