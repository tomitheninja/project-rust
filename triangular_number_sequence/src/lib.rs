/// A triangular number pair includes two information.
///
/// The first one is the width of a triangle (same as index)
/// The seconds it the number of points a equilateral triangle with the given width would contain
///
/// The value can be calculated like a factorial, but with the sum instead the product of the numbers
/// ## Example
/// The 5th triangular number is 1 + 2 + 3 + 4 + 5 = 15
#[derive(Clone, Default, Debug)]
pub struct TriangularNumber {
    index: usize,
    value: u64,
}

impl TriangularNumber {
    pub fn get_index(&self) -> usize {
        self.index
    }

    pub fn get_value(&self) -> u64 {
        self.value
    }
}

impl Iterator for TriangularNumber {
    type Item = Self;
    fn next(&mut self) -> Option<Self::Item> {
        let prev_state = self.clone();
        self.index += 1;
        self.value += self.index as u64;
        Some(prev_state)
    }
}

#[cfg(test)]
mod triangular_number {
    use super::*;

    #[test]
    fn initial_index_is_zero() {
        assert_eq!(0, TriangularNumber::default().get_index());
    }

    #[test]
    fn initial_value_is_zero() {
        assert_eq!(0, TriangularNumber::default().get_index());
    }
    #[test]
    fn index_matches() {
        let index = 10;
        assert_eq!(
            index,
            TriangularNumber::default().nth(index).unwrap().get_index()
        );
    }
    #[test]
    fn test_third() {
        assert_eq!(
            0 + 1 + 2 + 3,
            TriangularNumber::default().nth(3).unwrap().get_value()
        );
    }
}
