pub struct CollatzSequence {
    value: u64,
}

impl CollatzSequence {
    pub fn new(starting_value: u64) -> CollatzSequence {
        CollatzSequence {
            value: starting_value,
        }
    }
}

fn next_collatz_item(n: u64) -> Option<u64> {
    if n < 2 {
        None
    } else if n % 2 == 0 {
        Some(n / 2)
    } else {
        Some(3 * n + 1)
    }
}

impl Iterator for CollatzSequence {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        match next_collatz_item(self.value) {
            None => None,
            Some(next_value) => {
                self.value = next_value;
                Some(next_value)
            }
        }
    }
}

#[cfg(test)]
mod test_collatz_sequence {
    use super::*;

    #[test]
    fn test_next_collatz_item() {
        assert_eq!(40, next_collatz_item(13).unwrap());
        assert_eq!(20, next_collatz_item(40).unwrap());
        assert_eq!(10, next_collatz_item(20).unwrap());
        assert_eq!(5, next_collatz_item(10).unwrap());
        assert_eq!(16, next_collatz_item(5).unwrap());
        assert_eq!(8, next_collatz_item(16).unwrap());
        assert_eq!(4, next_collatz_item(8).unwrap());
        assert_eq!(2, next_collatz_item(4).unwrap());
        assert_eq!(1, next_collatz_item(2).unwrap());
        assert_eq!(None, next_collatz_item(1))
    }

    #[test]
    fn test_collatz_sequence_length() {
        assert_eq!(9, CollatzSequence::new(13).count());
    }
}
