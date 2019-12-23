// Sum square difference

// The sum of the squares of the first ten natural numbers is,
//  1^2 + 2^2 + ... + 10^2 = 385

// The square of the sum of the first ten natural numbers is,
// (1+2+...+10)^2 = 55^2 = 3025

// Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is 3025 − 385 = 2640.

// Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.

#[allow(dead_code)]
pub fn run_rust(n: u64) -> u64 {
    let sum_of_numbers_squared: u64 = (1..=n).map(|x| x * x).sum();
    let sum_of_numbers: u64 = (1..=n).sum();
    let square_of_sum_of_numbers = sum_of_numbers * sum_of_numbers;
    let diff = square_of_sum_of_numbers - sum_of_numbers_squared;

    diff
}

#[allow(dead_code)]
pub fn run_math(n: u64) -> u64 {
    let sum_of_numbers_squared = n * (n + 1) * (2 * n + 1) / 6;
    let sum_of_numbers = n * (n + 1) / 2;
    let square_of_sum_of_numbers = sum_of_numbers * sum_of_numbers;
    let diff = square_of_sum_of_numbers - sum_of_numbers_squared;

    diff
}

#[test]
fn test_math () {
    assert_eq!(2640, run_math(10));
    assert_eq!(25164150, run_math(100));
}

#[test]
fn test_rust () {
    assert_eq!(2640, run_rust(10));
    assert_eq!(25164150, run_rust(100));
}
