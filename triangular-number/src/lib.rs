/// A triangular number is made by adding all integers from 1 to N together.
///
/// Use it's iterator to generate a sequence
#[derive(Debug, Clone, PartialEq)]
pub struct TriangularNumber {
    /// sum of previous indexes
    value: u64,
    index: u64,
}

impl TriangularNumber {
    /// Creates a new `TriangularNumber` generator
    pub fn new() -> Self {
        TriangularNumber { value: 0, index: 0 }
    }

    /// Returns the next triangular number
    pub fn next_item(&self) -> u64 {
        self.value + self.index + 1
    }

    /// Generates the next triangular number, by mutating `self`
    pub fn into_next(&mut self) {
        self.value = self.next_item();
        self.index += 1;
    }
}

impl Iterator for TriangularNumber {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.into_next();
        Some(self.value)
    }
}

#[cfg(test)]
mod tests {

    use super::TriangularNumber;

    #[test]
    fn first_is_one() {
        assert_eq!(Some(1), TriangularNumber::new().nth(0));
    }

    #[test]
    fn second_is_three() {
        assert_eq!(Some(3), TriangularNumber::new().nth(1));
    }

    #[test]
    fn third_is_six() {
        assert_eq!(Some(6), TriangularNumber::new().nth(2));
    }

    #[test]
    fn fifth_is_fifteen() {
        assert_eq!(Some(15), TriangularNumber::new().nth(4));
    }
}
