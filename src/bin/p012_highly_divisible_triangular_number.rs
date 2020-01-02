// Highly divisible triangular number

// The sequence of triangle numbers is generated by adding the natural numbers. So the 7th triangle number would be 1 + 2 + 3 + 4 + 5 + 6 + 7 = 28. The first ten terms would be:
// 1, 3, 6, 10, 15, 21, 28, 36, 45, 55, ...

// Let us list the factors of the first seven triangle numbers:

// 1: 1
// 3: 1, 3
// 6: 1, 2, 3, 6
// 10: 1, 2, 5, 10
// 15: 1, 3, 5, 15
// 21: 1, 3, 7, 21
// 28: 1, 2, 4, 7, 14, 28

// We can see that 28 is the first triangle number to have over five divisors.

// What is the value of the first triangle number to have over five hundred divisors?

extern crate triangular_number;
use triangular_number::TriangularNumber;

fn count_dividers(n: u64) -> usize {
    (1..=(n as f64).sqrt() as u64)
        .filter(|&i| n % i == 0)
        .fold(0, |count, i| if n / i == i { count + 1 } else { count + 2 }) as usize
}

fn compute(min_count_divisors: usize) -> u64 {
    TriangularNumber::new()
        .map(|t_num| t_num.get_value())
        .filter(|&x| count_dividers(x) >= min_count_divisors)
        .nth(0)
        .unwrap()
}

fn main() {
    println!("p012: {}", compute(500));
}

#[cfg(test)]
mod test_p012 {
    use super::*;

    #[test]
    fn has_at_least_5_divisors() {
        assert_eq!(28, compute(5));
    }
    #[test]
    fn has_at_least_500_divisors() {
        assert_eq!(6500, compute(500) % 10000)
    }
}