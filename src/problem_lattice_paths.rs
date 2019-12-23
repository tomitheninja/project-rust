// Starting in the top left corner of a 2×2 grid, and only being able to move to the right and down, there are exactly 6 routes to the bottom right corner.

// How many such routes are there through a 20×20 grid?

fn factorial(n: u16) -> f64 {
    let mut result = 1.0;
    for i in 1..=n {
        result *= i as f64;
    }
    result
}

#[allow(dead_code)]
pub fn run(n: u16) -> f64 {
    factorial(n + n) / factorial(n) / factorial(n)
}

#[test]
fn test() {
    assert_eq!(run(2), 6.0);
    assert_eq!(run(20), 137846528820.0);
}
