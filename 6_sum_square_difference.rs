// Sum square difference

// The sum of the squares of the first ten natural numbers is,
//  1^2 + 2^2 + ... + 10^2 = 385

// The square of the sum of the first ten natural numbers is,
// (1+2+...+10)^2 = 55^2 = 3025

// Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is 3025 − 385 = 2640.

// Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.

fn run_rust(n: u64) {
    let sum_of_square_numbers: u64 = (1..=n).map(|x| x * x).sum();
    let sum_of_numbers: u64 = (1..=n).sum();
    let square_sum_of_numbers = sum_of_numbers * sum_of_numbers;

    println!(
        "with loops: {}) {} - {} = {}",
        n,
        square_sum_of_numbers,
        sum_of_square_numbers,
        square_sum_of_numbers - sum_of_square_numbers
    );
}

fn run_math(n: u64) {
    let sum_of_square_numbers = n * (n + 1) * (2 * n + 1) / 6;
    let sum_of_numbers = n * (n + 1) / 2;
    let square_sum_of_numbers = sum_of_numbers * sum_of_numbers;

    println!(
        "with math: {}) {} - {} = {}",
        n,
        square_sum_of_numbers,
        sum_of_square_numbers,
        square_sum_of_numbers - sum_of_square_numbers
    );
}

fn main() {
    let n = 100;
    run_rust(n);
    run_math(n);
}
