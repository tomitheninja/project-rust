//! Prime is a positive integer that can only be divided by one and itself
//! The first prime numbers are ```2, 3, 5, 7, 11```

mod factors;
mod is_prime;
mod many;
pub mod sieve_of_eratosthenes;

pub use factors::prime_factors;
pub use is_prime::is_prime;
pub use many::many_primes_below;
// pub use sieve_of_eratosthenes;
