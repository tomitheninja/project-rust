// Largest prime factor

// The prime factors of 13195 are 5, 7, 13 and 29.

// What is the largest prime factor of the number 600851475143 ?

fn smallest_factor(n: u64) -> u64 {
    match n {
        1 => 1,
        _ => (2..)
            .filter(|&x| n % x == 0)
            .nth(0).unwrap()
    }
}

fn main() {
    let mut n = 600851475143;
    let mut latest_factor = 0;
    while n > 1 {
        latest_factor = smallest_factor(n);
        n /= latest_factor;
    };
    // smallest_factor()'s last output is the largest one 
    println!("{}", latest_factor);
}
