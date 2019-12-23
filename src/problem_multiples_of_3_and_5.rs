// Multiples of 3 and 5

// If we list all the natural numbers below 10
// that are multiples of 3 or 5,
// we get 3, 5, 6 and 9.
// The sum of these multiples is 23.

// Find the sum of all the multiples of 3 or 5 below 1000.

#[allow(dead_code)]
pub fn run (n: usize) -> u32 {
    (1..n as u32)
        .filter(|&x| x % 3 == 0 || x % 5 == 0)
        .sum()
}

#[test]
fn test () {
    assert_eq!(233168, run(1000));
}
