//! The fibonacci sequence is a mathematical sequence.
//!
//! Each item in the sequence is the sum of the two preceding ones.
//!
//! The sequence starts like this: `[1, 1, 2, 3, 5, 8, ...]`
//!
//! This implementation is indexed from zero

/// A type definition for the fibonacci sequence
///
/// The `FibonacciSequence` type allows basic operations on the fibonacci numbers
/// # Examples
///
/// ```rust
/// use fibonacci_sequence::FibonacciSequence;
///
/// let result = FibonacciSequence::new().nth(4);
/// assert_eq!(Some(5), result);
/// ```
#[derive(Debug, Clone)]
pub struct FibonacciSequence {
    value: u64,
    prev_value: u64,
}

impl FibonacciSequence {
    pub fn new() -> Self {
        FibonacciSequence {
            value: 1,
            prev_value: 0,
        }
    }

    pub fn get_value(&self) -> u64 {
        self.value
    }

    /// Calculate the next value, return the result
    pub fn next_value(&self) -> u64 {
        self.value + self.prev_value
    }

    /// Calculate the next value and save the result
    pub fn into_next_value(&mut self) {
        let (pv, v) = (self.value, self.next_value());
        self.prev_value = pv;
        self.value = v;
    }
}

impl Iterator for FibonacciSequence {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.get_value();
        self.into_next_value();
        Some(result)
    }
}

#[cfg(test)]
mod fibonacci_sequence {
    use super::FibonacciSequence;

    mod interface {
        use super::*;

        #[test]
        fn initial_value_is_one() {
            let fib = FibonacciSequence::new();
            assert_eq!(1, fib.get_value());
        }

        #[test]
        fn next_value_is_one() {
            let fib = FibonacciSequence::new();
            assert_eq!(1, fib.next_value());
        }

        mod into_next {
            use super::*;
            #[test]
            fn value_is_one() {
                let mut fib = FibonacciSequence::new();
                fib.into_next_value();
                assert_eq!(1, fib.get_value());
            }

            #[test]
            fn next_value_is_two() {
                let mut fib = FibonacciSequence::new();
                fib.into_next_value();
                assert_eq!(2, fib.next_value());
            }
        }
    }

    mod iterator {
        use super::*;

        #[test]
        fn first_is_one() {
            assert_eq!(Some(1), FibonacciSequence::new().nth(0));
        }

        #[test]
        fn second_is_one() {
            assert_eq!(Some(1), FibonacciSequence::new().nth(1));
        }

        #[test]
        fn third_is_two() {
            assert_eq!(Some(2), FibonacciSequence::new().nth(2));
        }

        #[test]
        fn sixth_is_eight() {
            assert_eq!(Some(8), FibonacciSequence::new().nth(5));
        }
    }
}
