// makes a guess about how many primes are there below n
pub fn many_primes_below(n: usize) -> f64 {
    use std::f64::consts::E;
    let m = n as f64;
    m / m.log(E)
}

#[cfg(test)]
mod many_primes {
    use super::many_primes_below;

    #[test]
    fn approx_168_below_thousand() {
        const THOUSAND: usize = 1_000;
        let many = many_primes_below(THOUSAND);
        assert!((168.0 - many).abs() < 25.0);
    }

    #[test]
    fn approx_78498_below_one_million() {
        const ONE_MILLION: usize = 1_000_000;
        let many = many_primes_below(ONE_MILLION);
        assert!((78498.0 - many).abs() < 10_000.0);
    }
}
