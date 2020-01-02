pub struct Fibonacci(u32, u32);

impl Fibonacci {

    fn get_next_pair(&self) -> Self {
        Fibonacci(self.1, self.0 + self.1)
    }
}

impl Default for Fibonacci {
    fn default() -> Self {
        Fibonacci(0, 1)
    }
}

impl Iterator for Fibonacci {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        let result = self.0;
        let next_state = self.get_next_pair();
        self.0 = next_state.0;
        self.1 = next_state.1;
        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::Fibonacci;
    #[test]
    fn starts_with_zero() {
        let f = Fibonacci::new();
        assert_eq!(f.0, 0);
    }
    #[test]
    fn continues_with_one() {
        let f = Fibonacci::new();
        assert_eq!(f.1, 1);
    }
    #[test]
    fn iterator_stars_with_zero() {
        let x = Fibonacci::new().nth(0).unwrap();
        assert_eq!(x, 0);
    }
    #[test]
    fn iterator_first_is_1() {
        let x = Fibonacci::new().nth(1).unwrap();
        assert_eq!(x, 1);
    }
    #[test]
    fn iterator_second_is_1() {
        let x = Fibonacci::new().nth(2).unwrap();
        assert_eq!(x, 1);
    }
    #[test]
    fn iterator_third_is_2() {
        let x = Fibonacci::new().nth(3).unwrap();
        assert_eq!(x, 2);
    }
    #[test]
    fn iterator_forth_is_3() {
        let x = Fibonacci::new().nth(4).unwrap();
        assert_eq!(x, 3);
    }
}
