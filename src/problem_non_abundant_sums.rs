// Non-abundant sums

// A perfect number is a number for which the sum of its proper divisors is exactly equal to the number.
// For example, the sum of the proper divisors of 28 would be 1 + 2 + 4 + 7 + 14 = 28, which means that 28 is a perfect number.

// A number n is called deficient if the sum of its proper divisors is less than n
// and it is called abundant if this sum exceeds n.

// As 12 is the smallest abundant number,
// 1 + 2 + 3 + 4 + 6 = 16, the smallest number that can be written as the sum of two abundant numbers is 24.
// By mathematical analysis, it can be shown that all integers greater than 28123 can be written as the sum of two abundant numbers.
// However, this upper limit cannot be reduced any further by analysis even though it is known that the greatest number that cannot be expressed as the sum of two abundant numbers is less than this limit.

// Find the sum of all the positive integers which cannot be written as the sum of two abundant numbers.

fn sum_of_unique_params(n1: u32, n2: u32) -> u32 {
    if n1 == n2 {
        n1
    } else {
        n1 + n2
    }
}

fn sum_of_divisors(n: u32) -> u32 {
    (2..=(n as f64).sqrt() as u32)
        .filter(|&i| n % i == 0)
        .fold(1, |sum, div| sum + sum_of_unique_params(div, n / div))
}

fn get_abundant_numbers_below_limit(limit: u32) -> Vec<u32> {
    return (1..limit)
        .filter(|i| *i < sum_of_divisors(*i))
        .collect::<Vec<u32>>();
}

fn is_num_sum_of_two(sum: &u32, a: &Vec<u32>) -> bool {
    for item1 in a {
        if item1 > sum {
            break;
        };
        let item2_candidate = sum - item1; // n2 = sum - n1
                                           // if *item1 == item2_candidate {
                                           //     continue
                                           // };
        match a.binary_search(&item2_candidate) {
            Err(_index) => continue,
            Ok(_index) => return true,
        }
    }
    return false;
}

#[allow(dead_code)]
pub fn run(limit: u32) -> u32 {
    let abundant_numbers_below_limit = get_abundant_numbers_below_limit(limit);

    (1..limit)
        .filter(|i| !is_num_sum_of_two(i, &abundant_numbers_below_limit))
        .sum()
}

#[test]
fn test() {
    assert_eq!(run(28124), 4179871);
}
