// Quadratic primes
// Problem 27

// Euler discovered the remarkable quadratic formula:
// n2+n+41
// if 0 <= n <= 39; the result is a prime number
// find a formula like this: n^2 * a*n + b
// where |a| < 1000 && |b| < 1000
// Find the product of the coefficients, a and b, for the quadratic expression that produces the maximum number of primes for consecutive values of n, starting with n=0.

fn is_prime(x: i32) -> bool {
    x > 1 && !(2..x).any(|d| x % d == 0)
}

fn compute(limit: i32) -> (usize, i32, i32) {
    let mut best: (usize, i32, i32) = (0, -1, -1);
    for a in -limit..limit {
        for b in -limit..limit {
            let len = (0..)
                .map(|n| n * n + a * n + b)
                .map(|x| is_prime(x))
                .take_while(|x: &bool| *x)
                .count();
            if len > best.0 {
                best = (len, a, b);
            }
        }
    }
    best
}

fn main() {
    let (_len, a, b) = compute(1_000);
    println!("{}", a * b);
}

#[test]
fn compute_1000() {
    let (_len, a, b) = compute(1000);
    assert_eq!(a * b % 100, -31);
}
