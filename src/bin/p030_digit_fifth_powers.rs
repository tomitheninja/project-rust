// Digit fifth powers
// Problem 30

// Surprisingly there are only three numbers that can be written as the sum of fourth powers of their digits:
//     1634 = 1^4 + 6^4 + 3^4 + 4^4
//     8208 = 8^4 + 2^4 + 0^4 + 8^4
//     9474 = 9^4 + 4^4 + 7^4 + 4^4
// As 1 = 1^4 is not a sum it is not included.
// The sum of these numbers is 1634 + 8208 + 9474 = 19316.
// Find the sum of all the numbers that can be written as the sum of fifth powers of their digits.

fn sum_digits_pow(digits: u32, pow: u32) -> u32 {
    let mut n = digits;
    let mut sum = 0;
    while n != 0 {
        let last_digit = n % 10;
        sum += last_digit.pow(pow as u32);
        n /= 10;
    }
    sum
}

fn compute(pow: u32) -> u32 {
    (2..1_000_000_u32)
        .filter(|x| *x == sum_digits_pow(*x, pow))
        .sum()
}

fn main() {
    println!("sum: {}", compute(5));
}

#[test]
fn compute_4() {
    assert_eq!(19316, compute(4));
}

#[test]
fn compute_5() {
    assert_eq!(39, compute(5) % 100);
}
