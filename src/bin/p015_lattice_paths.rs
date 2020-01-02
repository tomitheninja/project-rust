// Starting in the top left corner of a 2×2 grid, and only being able to move to the right and down, there are exactly 6 routes to the bottom right corner.

// How many such routes are there through a 20×20 grid?

fn factorial(n: u16) -> f64 {
    (1..=n)
        .map(|i| i as f64)
        .product()
}

pub fn compute(n: u16) -> f64 {
    factorial(n + n) / factorial(n) / factorial(n)
}

pub fn main () {
    println!("p015: {}", compute(20));
}

#[cfg(test)]
mod test_p015 {
    use super::*;

    #[test]
    fn factorial_5() {
        assert_eq!(120.0, factorial(5));
    }
    #[test]
    fn solve_for_2() {
        assert_eq!(6.0, compute(2));
    }

    #[test]
    fn solve_for_20() {
        assert_eq!(820.0, compute(20) % 1000.0);
    }
}
