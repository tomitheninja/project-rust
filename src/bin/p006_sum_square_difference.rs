// Sum square difference

// The sum of the squares of the first ten natural numbers is,
//  1^2 + 2^2 + ... + 10^2 = 385

// The square of the sum of the first ten natural numbers is,
// (1+2+...+10)^2 = 55^2 = 3025

// Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is 3025 − 385 = 2640.

// Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.

fn get_sum_of_numbers_in_range(n: u64) -> u64 {
    n * (n + 1) / 2
}

fn get_sum_of_squares_in_range(n: u64) -> u64 {
    n * (n + 1) * (2 * n + 1) / 6
}

pub fn compute_math(n: u64) -> u64 {
    let a = get_sum_of_numbers_in_range(n);
    let b = get_sum_of_squares_in_range(n);

    a * a - b
}

pub fn compute_rust(n: u64) -> u64 {
    let a: u64 = (1..=n).sum();
    let b: u64 = (1..=n).map(|x| x * x).sum();

    a * a - b
}

pub fn main () {
    println!("p006: {}", compute_math(100));
}

#[cfg(test)]
mod p006_tests {
    use super::*;

    #[test]
    fn solve_for_10_with_math() {
        assert_eq!(2640, compute_math(10));
    }
    #[test]
    fn solve_for_10_with_rust() {
        assert_eq!(2640, compute_rust(10));
    }
    #[test]
    fn sum_of_numbers_in_range_1_10() {
        assert_eq!(55, get_sum_of_numbers_in_range(10));
    }
    #[test]
    fn sum_of_squared_numbers_in_range_1_10() {
        assert_eq!(385, get_sum_of_squares_in_range(10));
    }
    #[test]
    fn solve_for_100_with_math() {
        assert_eq!(50, compute_math(100) % 100);
    }
}
