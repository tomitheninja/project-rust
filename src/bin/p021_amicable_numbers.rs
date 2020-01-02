// Amicable numbers

// Let d(n) be defined as the sum of proper divisors of n (numbers less than n which divide evenly into n).
// If d(a) = b and d(b) = a, where a ≠ b, then a and b are an amicable pair and each of a and b are called amicable numbers.

// For example, the proper divisors of 220 are 1, 2, 4, 5, 10, 11, 20, 22, 44, 55 and 110; therefore d(220) = 284.
// The proper divisors of 284 are 1, 2, 4, 71 and 142; so d(284) = 220.

// Evaluate the sum of all the amicable numbers under 10000.

// performance: O=log(n)
fn d(n: u32) -> u32 {
    (2..=(n as f64).sqrt() as u32)
        .filter(|&i| n % i == 0)
        .fold(1, |sum, div| sum + div + n / div)
}

fn compute(bound: u32) -> u32 {
    (1..bound)
        .filter(|a| d(*a) != *a)
        .filter(|a| d(d(*a)) == *a)
        .sum::<u32>()
}

fn main() {
    println!("p021: {}", compute(10_000));
}

#[cfg(test)]
mod test_d {
    use super::*;

    #[test]
    fn d_220() {
        assert_eq!(284, d(220));
    }
    #[test]
    fn d_284() {
        assert_eq!(220, d(284));
    }
}

#[cfg(test)]
mod test_sum {
    use super::*;

    #[test]
    fn below_10_000 () {
        assert_eq!(626, compute(10_000) % 1000);
    }
}
