// Longest Collatz sequence

// The following iterative sequence is defined for the set of positive integers:

// n → n/2 (n is even)
// n → 3n + 1 (n is odd)

// Using the rule above and starting with 13, we generate the following sequence:
// 13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1

// It can be seen that this sequence (starting at 13 and finishing at 1) contains 10 terms. Although it has not been proved yet (Collatz Problem), it is thought that all starting numbers finish at 1.

// Which starting number, under one million, produces the longest chain?

// NOTE: Once the chain starts the terms are allowed to go above one million.

fn is_even (n: u64) -> bool {
    return n%2 == 0;
}

fn next_collatz_item(n: u64) -> Option<u64> {
    if n < 2 {
        None
    } else if is_even(n) {
        Some(n / 2)
    } else {
        Some(3 * n + 1)
    }
}

struct CollatzSequence {
    value: u64,
}

impl CollatzSequence {
    fn new (starting_value: u64) -> CollatzSequence {
        CollatzSequence { value: starting_value }
    }
}

impl Iterator for CollatzSequence {
    type Item = u64;
    fn next (&mut self) -> Option<Self::Item> {
        match next_collatz_item(self.value) {
            None => None,
            Some(next_value) => {
                self.value = next_value;
                Some(next_value)
            },
        }
    }
}

fn main () {
    let result = (1..1_000_000)
        .max_by_key(|starting_value| CollatzSequence::new(*starting_value).count())
        .unwrap();
    println!("{}", result);

    assert_eq!(result, 837799);
}
