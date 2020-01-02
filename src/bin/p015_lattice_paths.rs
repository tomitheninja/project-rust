// Starting in the top left corner of a 2×2 grid, and only being able to move to the right and down, there are exactly 6 routes to the bottom right corner.

// How many such routes are there through a 20×20 grid?

fn factorial(n: u16) -> f64 {
    (1..=n).map(|i| i as f64).product()
}

pub fn compute(n: u16) -> f64 {
    factorial(n + n) / factorial(n) / factorial(n)
}

pub fn main() {
    println!("p015: {}", compute(20));
}

#[cfg(test)]
mod test_p015 {
    use super::*;
    const EPSILON: f64 = 0.1;

    #[test]
    fn factorial_5() {
        assert!((factorial(5) - 120.0).abs() < EPSILON);
    }
    #[test]
    fn solve_for_2() {
        assert!((compute(2) - 6.0).abs() < EPSILON);
    }

    #[test]
    fn solve_for_20() {
        let result = compute(20);
        let diff = (result % 1000.0) - 820.0;
        assert!(diff.abs() < EPSILON);
    }
}
