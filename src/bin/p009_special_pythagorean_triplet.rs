// Special Pythagorean triplet

// A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
// a^2 + b^2 = c^2
// For example, 3^2 + 4^2 = 9 + 16 = 25 = 5^2.

// There exists exactly one Pythagorean triplet for which a + b + c = 1000.
// Find the product abc.

fn sqrt(x: u32) -> Option<u32> {
    let result = (x as f64).sqrt() as u32;
    if result * result == x {
        Some(result)
    } else {
        None
    }
}

fn compute(sum: u32) -> Option<u32> {
    for a in 1..sum {
        for b in 1..=a {
            let maybe_c = sqrt(a * a + b * b);
            let c = match maybe_c {
                None => continue,
                Some(c) => c,
            };
            if a + b + c == sum && a * a + b * b == c * c {
                println!("{} {} {}", a, b, c);
                return Some(a * b * c);
            }
        }
    }
    None
}

fn main() {
    println!("p010: {}", compute(1_000).unwrap());
}

#[cfg(test)]
mod p009_tests {
    use super::*;

    #[test]
    fn test_for_3_4_and_5() {
        assert_eq!(3 * 4 * 5, compute(3 + 4 + 5).unwrap());
    }

    #[test]
    fn test_for_1000() {
        assert_eq!(75_000, compute(1000).unwrap() % 100_000);
    }
}
