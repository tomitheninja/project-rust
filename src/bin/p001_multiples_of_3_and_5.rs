// Multiples of 3 and 5

// If we list all the natural numbers below 10
// that are multiples of 3 or 5,
// we get 3, 5, 6 and 9.
// The sum of these multiples is 23.

// Find the sum of all the multiples of 3 or 5 below 1000.

fn compute(bound: usize) -> u32 {
    (1..bound as u32)
        .filter(|&x| x % 3 == 0 || x % 5 == 0)
        .sum()
}

fn main() {
    println!("p001: {}", compute(1000));
}

#[cfg(test)]
mod p001_tests {
    use super::compute;

    #[test]
    fn below_ten() {
        assert_eq!(23, compute(10));
    }

    #[test]
    fn below_thousand() {
        assert_eq!(68, compute(1000) % 100);
    }
}
