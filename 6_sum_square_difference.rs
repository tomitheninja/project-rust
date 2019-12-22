// Sum square difference

// The sum of the squares of the first ten natural numbers is,
//  1^2 + 2^2 + ... + 10^2 = 385

// The square of the sum of the first ten natural numbers is,
// (1+2+...+10)^2 = 55^2 = 3025

// Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is 3025 − 385 = 2640.

// Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.

fn run_rust(n: u64) {
    let sum_of_numbers_squared: u64 = (1..=n).map(|x| x * x).sum();
    let sum_of_numbers: u64 = (1..=n).sum();
    let square_of_sum_of_numbers = sum_of_numbers * sum_of_numbers;
    let diff = square_of_sum_of_numbers - sum_of_numbers_squared;

    assert_eq!(square_of_sum_of_numbers, 25502500);
    assert_eq!(sum_of_numbers_squared, 338350);
    assert_eq!(diff, 25164150);

    println!(
        "with loops: {}) {} - {} = {}",
        n,
        square_of_sum_of_numbers,
        sum_of_numbers_squared,
        diff,
    );
}

fn run_math(n: u64) {
    let sum_of_numbers_squared = n * (n + 1) * (2 * n + 1) / 6;
    let sum_of_numbers = n * (n + 1) / 2;
    let square_of_sum_of_numbers = sum_of_numbers * sum_of_numbers;
    let diff = square_of_sum_of_numbers - sum_of_numbers_squared;

    assert_eq!(square_of_sum_of_numbers, 25502500);
    assert_eq!(sum_of_numbers_squared, 338350);
    assert_eq!(diff, 25164150);

    println!(
        "with math: {}) {} - {} = {}",
        n,
        square_of_sum_of_numbers,
        sum_of_numbers_squared,
        diff,
    );
}

fn main() {
    const N: u64 = 100;
    run_rust(N);
    run_math(N);
}
