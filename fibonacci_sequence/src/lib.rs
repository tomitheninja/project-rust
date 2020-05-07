/// generates the fibonacci sequence
///
/// ## Example
/// ```
/// use fibonacci_sequence::Fibonacci;
///
/// assert_eq!(Fibonacci::new().nth(0).unwrap(), 0);
/// assert_eq!(Fibonacci::new().nth(1).unwrap(), 1);
/// assert_eq!(Fibonacci::new().nth(2).unwrap(), 1);
/// assert_eq!(Fibonacci::new().nth(6).unwrap(), 8);
/// ```
#[derive(Debug, PartialEq)]
pub struct Fibonacci {
    pub value: u64,
    next_value: u64,
}

impl Fibonacci {
    /// alias for default()
    pub fn new() -> Self {
        Fibonacci::default()
    }
}

impl Default for Fibonacci {
    fn default() -> Self {
        Fibonacci {
            value: 0,
            next_value: 1,
        }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let prev_value = self.value;
        self.value = self.next_value;
        self.next_value = prev_value + self.value;
        Some(prev_value)
    }
}

#[cfg(test)]
mod fibonacci_sequence {
    use super::*;

    #[cfg(test)]
    mod default_value {
        use super::*;

        #[test]
        fn with_default() {
            assert_eq!(Fibonacci::default().value, 0);
        }

        #[test]
        fn with_new() {
            assert_eq!(Fibonacci::new().value, 0);
        }
    }
}
