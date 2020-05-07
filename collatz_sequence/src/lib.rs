/// A sequence defined as follows:
///
/// Start with any positive integer n
///
/// If the value is odd, the next item is three times the value plus 1
///
/// If the value is even, the next item is the half of the value
///
/// Repeat the last two steps, until the sequence reaches 1
#[derive(Debug, PartialEq)]
pub struct CollatzSequence {
    value: u64,
}

impl CollatzSequence {
    pub fn new(initial_value: u64) -> Self {
        CollatzSequence {
            value: initial_value,
        }
    }

    fn next_item(n: u64) -> Option<u64> {
        if n < 2 {
            None
        } else if n % 2 == 0 {
            Some(n / 2)
        } else {
            Some(3 * n + 1)
        }
    }
}

impl Iterator for CollatzSequence {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        match CollatzSequence::next_item(self.value) {
            None => None,
            Some(next_value) => {
                self.value = next_value;
                Some(next_value)
            }
        }
    }
}

#[cfg(test)]
mod collatz_sequence {
    use super::*;

    #[cfg(test)]
    mod even_value {
        use super::*;

        #[test]
        fn halves_6() {
            assert_eq!(CollatzSequence::new(6).next().unwrap(), 3);
        }

        #[test]
        fn halves_2() {
            assert_eq!(CollatzSequence::new(2).next().unwrap(), 1);
        }
    }

    #[cfg(test)]
    mod odd_value {
        use super::*;

        #[test]
        fn multiplies_and_increases_11() {
            assert_eq!(CollatzSequence::new(11).next().unwrap(), 11 * 3 + 1);
        }

        #[test]
        fn multiplies_and_increases_3() {
            assert_eq!(CollatzSequence::new(3).next().unwrap(), 3 * 3 + 1);
        }
    }

    #[test]
    fn test_collatz_sequence_length() {
        // 10, 5, 16, 8, 4, 2, 1
        assert_eq!(CollatzSequence::new(3).count(), 7);
    }
}
