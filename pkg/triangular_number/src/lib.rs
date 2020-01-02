#[derive(Clone, Default)]
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
    type Item = TriangularNumber;
    fn next(&mut self) -> Option<Self::Item> {
        let prev_state = self.clone();
        self.index += 1;
        self.value += self.index as u64;
        Some(prev_state)
    }
}

#[cfg(test)]
mod tests_triangular_number {
    use super::*;

    #[test]
    fn initial_index_is_zero() {
        assert_eq!(0, TriangleNumber::new().get_index());
    }
    #[test]
    fn initial_value_is_zero() {
        assert_eq!(0, TriangleNumber::new().get_index());
    }
    #[test]
    fn index_matches() {
        let index = 10;
        assert_eq!(index, TriangleNumber::new().nth(index).unwrap().get_index());
    }
    #[test]
    fn test_third() {
        assert_eq!(0 + 1 + 2 + 3, TriangleNumber::new().nth(3).unwrap().get_value());
    }
}
