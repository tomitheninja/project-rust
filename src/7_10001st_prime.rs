// 10001st prime

// By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.

// What is the 10 001st prime number?

fn get_prime_table(n: usize) -> Vec<bool> {
    assert!(n > 2);
    let mut v = vec![true; n];
    v[0] = false;
    v[1] = false;

    for step in 2..n {
        if !v[step] {
            continue;
        } // Already false
        for factor in (2 * step..n).step_by(step) {
            v[factor] = false;
        }
    }
    v
}

fn main() {
    let x = get_prime_table(999999);
    let mut num_found_remaining = 10001;
    for i in 0..x.len() {
        if x[i] {
            num_found_remaining -= 1;
            if num_found_remaining == 0 {
                println!("{} ", i);
                break;
            }
        }
    }
}
