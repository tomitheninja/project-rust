// Smallest multiple

// 2510 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.

// What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 10?

fn smallest_factor(n: u32) -> u32 {
    match n {
        1 => 1,
        _ => (2..).filter(|&x| n % x == 0).take(1).sum(), // get first
    }
}

/// Returns a vector that contains the factors of a range
///
/// Example (range: 4)
/// - count of **1**: 1 - *a natural number (there can be infinity many in a number)*
/// - count of **2**: 2 - *there are 1 in '2' and 2 in '4'*
/// - count of **3**: 1 - *in '3'*
/// - count of **4**: 0 - *since it is 2x2*
fn get_largest_factors_in_range(len: usize) -> Vec<u32> {
    let mut largest_factors = {
        let mut vec_with_n_zeros = vec![0; len];
        vec_with_n_zeros[0] = 1;
        vec_with_n_zeros
    };

    for n in 2..=largest_factors.len() {
        let mut num = n as u32;
        let mut prev_factor = 1;
        let mut prev_factor_amount = 0;
        loop {
            let factor = smallest_factor(num);
            // println("{} is factor of {}", factor, n)
            if factor == prev_factor {
                prev_factor_amount += 1;
            } else {
                let max_amount_of_factor = largest_factors[(prev_factor - 1) as usize];
                if prev_factor_amount > max_amount_of_factor {
                    largest_factors[(prev_factor - 1) as usize] = prev_factor_amount
                }
                prev_factor = factor;
                prev_factor_amount = 1;
            }
            num /= factor;
            if prev_factor == 1 && prev_factor_amount > 2 {
                break;
            }
        }
    }
    largest_factors
}

fn get_product_of_factors(factor_amount: Vec<u32>) -> u64 {
    let mut product: u64 = 1;
    for i in 1..=factor_amount.len() {
        let factor = i;
        let amount = factor_amount[i - 1];
        let tmp = factor.pow(amount);
        if tmp > 1 {
            // println!("{:02}^{:2} = {:2}", i, factor_amount[i - 1], tmp);
            product *= tmp as u64;
        }
    }
    product
}

fn main() {
    let n = 20;
    let factors = get_largest_factors_in_range(n);
    let result = get_product_of_factors(factors);
    println!("{}", result);
}