// Even Fibonacci numbers

// Each new term in the Fibonacci sequence is generated by adding the previous two terms.
// By starting with 1 and 2, the first 10 terms will be:

// 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...

// By considering the terms in the Fibonacci sequence whose values do not exceed four million,
// find the sum of the even-valued terms.

extern crate fibonacci;
use fibonacci::Fibonacci;

fn compute(bound: u32) -> u32 {
    Fibonacci::new()
        .take_while(|&x| x <= bound)
        .filter(|&x| x % 2 == 0)
        .sum()
}

fn main() {
    println!("p002: {}", compute(4_000_000));
}

#[cfg(test)]
mod p002_tests {
    use super::compute;

    #[test]
    fn below_hundred() {
        assert_eq!(2 + 8 + 34, compute(100));
    }
    #[test]
    fn below_four_million() {
        assert_eq!(32, compute(4_000_000) % 100);
    }
}