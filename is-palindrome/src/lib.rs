fn reverse_number(num: u64) -> u64 {
    let mut n = num;
    let mut rev = 0;
    while n > 0 {
        rev = rev * 10 + n % 10;
        n /= 10;
    }
    rev
}

pub fn num(value: u64) -> bool {
    value == reverse_number(value)
}

pub fn str(value: &str) -> bool {
    value.chars().rev().cmp(value.chars()) == std::cmp::Ordering::Equal
}

#[cfg(test)]
mod is_palindrome_tests {

    mod numbers {
        use super::super::num as is_palindrome;

        #[test]
        fn palindrome_numbers() {
            assert!(is_palindrome(1));
            assert!(is_palindrome(22));
            assert!(is_palindrome(12321));
            assert!(is_palindrome(123321));
        }

        #[test]
        fn not_palindrome_numbers() {
            assert!(!is_palindrome(12));
            assert!(!is_palindrome(123));
        }
    }

    mod strings {
        use super::super::str as is_palindrome;

        #[test]
        fn palindrome_strings() {
            assert!(is_palindrome("a"));
            assert!(is_palindrome("anna"));
            assert!(is_palindrome("indul a görög a ludni"))
        }

        #[test]
        fn not_palindrome_strings() {
            assert!(!is_palindrome("ab"));
            assert!(!is_palindrome("Aa"));
            assert!(!is_palindrome("foobar"))
        }
    }
}
