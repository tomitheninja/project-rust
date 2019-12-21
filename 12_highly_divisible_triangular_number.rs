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

/// Returns the number of dividers of n
/// Includes 1 and n
fn count_dividers(n: u64) -> usize {
    (1..=(n as f64).sqrt() as u64)
        .filter(|&i| n % i == 0)
        .fold(0, |count, i| if n / i == i { count + 1 } else { count + 2 }) as usize
}

/// Represents a number that calculated like this: 1+2+3+4...+n
struct TriangleNumber {
    value: u64,
    sum: u64,
}

impl TriangleNumber {
    fn new() -> TriangleNumber {
        TriangleNumber {
            value: 1,
            sum: 1,
        }
    }
}

impl Iterator for TriangleNumber {
    type Item = u64;
    fn next (&mut self) -> Option<u64> {
        self.value += 1;
        self.sum += self.value;
        Some(self.sum)
    }
}


fn main () {
    let result: u64 = TriangleNumber::new()
        .filter(|&x| count_dividers(x) >= 500)
        .nth(0).unwrap();
    println!("{}", result);
}
