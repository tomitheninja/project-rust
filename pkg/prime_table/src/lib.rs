pub fn get_prime_table(n: usize) -> Vec<bool> {
    assert!(n >= 2);
    let mut v = vec![true; n];
    v[0] = false;
    v[1] = false;

    for step in 2..n {
        if !v[step] {
            continue; // Skip if already false
        }
        for factor in (2 * step..n).step_by(step) {
            v[factor] = false;
        }
    }
    v
}

#[cfg(test)]
mod test_prime_table {
    use super::get_prime_table;

    #[test]
    fn test_0() {
        let x = get_prime_table(2);
        assert_eq!(false, x[0]);
    }
    #[test]
    fn test_1() {
        let x = get_prime_table(2);
        assert_eq!(false, x[1]);
    }
    #[test]
    fn test_2() {
        let x = get_prime_table(3);
        assert_eq!(true, x[2]);
    }
    #[test]
    fn test_4() {
        let x = get_prime_table(5);
        assert_eq!(false, x[4]);
    }
    #[test]
    fn test_5() {
        let x = get_prime_table(6);
        assert_eq!(true, x[5]);
    }
}
