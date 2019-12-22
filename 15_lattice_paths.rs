// Starting in the top left corner of a 2×2 grid, and only being able to move to the right and down, there are exactly 6 routes to the bottom right corner.

// How many such routes are there through a 20×20 grid?

fn factorial (n: u16) -> f64 {
    let mut result = 1.0;
    for i in 1..=n {
        result *= i as f64;
    }
    result
}

fn main () {
    const N: u16 = 20;
    let result = factorial(N + N)
        / factorial(N)
        / factorial(N);

    println!("{}", result);

    assert_eq!(result, 137846528820.0);
}
