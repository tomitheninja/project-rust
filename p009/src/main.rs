//! A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
//!  a<sup>2</sup> + b<sup>2</sup> = c<sup>2</sup>
//!
//! For example, 32 + 42 = 9 + 16 = 25 = 52.
//! There exists exactly one Pythagorean triplet for which a + b + c = 1000.
//! Find the product abc.

fn main() {
    println!("{}", (solve(1000).unwrap()));
}

fn pythagoras(a: u64, b: u64) -> Option<u64> {
    let c2 = a * a + b * b;
    let c = (c2 as f64).sqrt() as u64;
    if c * c == c2 {
        Some(c)
    } else {
        None
    }
}

fn solve(sum: u64) -> Option<u64> {
    for a in (1..=sum / 2).rev() {
        for b in (a..sum / 2).rev() {
            if let Some(c) = pythagoras(a, b) {
                if a + b + c == sum {
                    return Some(a * b * c);
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod p009 {
    use super::*;

    #[test]
    fn pythagoras_5_12_13() {
        assert_eq!(Some(13), pythagoras(5, 12));
    }

    #[test]
    fn solve_for_1000() {
        assert_eq!(Some(31875000), solve(1000));
    }
}
