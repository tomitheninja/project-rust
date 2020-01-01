// Largest palindrome product

// A palindromic number reads the same both ways.
// The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.

// Find the largest palindrome made from the product of two 3-digit numbers.

/// 10^(number of digits in n)
///
/// examples:
/// - 999 -> 100
/// - 1234 -> 1000
fn get_size_of_number(n: u32) -> u32 {
    let mut result = 1;
    while result * 10 <= n {
        result *= 10;
    }
    result
}

/// returns true if the number stays the same when reversed
fn is_palindrome(n: u32) -> bool {
    let mut num = n;
    let mut size_of_number = get_size_of_number(num);
    while num > 9 {
        let left_digit = num / size_of_number;
        let right_digit = num % 10;
        if left_digit != right_digit {
            return false;
        }
        num = num % size_of_number / 10; // remove digits from left and right
        size_of_number /= 100; // reduce by 2 digits
    }
    true
}

fn compute(range_first: u32, range_last: u32) -> u32 {
    (range_first..=range_last)
        .map(|num1| {
            (range_first..=range_last)
                .map(|num2| (num1 * num2) as u32)
                .filter(|product| is_palindrome(*product))
                .max()
                .unwrap_or(0)
        })
        .max()
        .unwrap()
}

fn main() {
    println!("p004: {}", compute(100, 999));
}

#[cfg(test)]
mod p004_tests {
    use super::{compute, is_palindrome};
    #[test]
    fn _1_is_palindrome() {
        assert_eq!(true, is_palindrome(1));
    }
    #[test]
    fn _10_is_not_palindrome() {
        assert_eq!(false, is_palindrome(10));
    }
    #[test]
    fn _101_is_palindrome() {
        assert_eq!(true, is_palindrome(101));
    }
    #[test]
    fn _111_is_palindrome() {
        assert_eq!(true, is_palindrome(111));
    }
    #[test]
    fn two_digit() {
        assert_eq!(9009, compute(10, 99));
    }
    #[test]
    fn three_digit() {
        assert_eq!(09, compute(100, 999) % 100);
    }
}
