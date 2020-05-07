// Coin sums
// Problem 31

// In the United Kingdom the currency is made up of pound (£) and pence (p). There are eight coins in general circulation:
//     1p, 2p, 5p, 10p, 20p, 50p, £1 (100p), and £2 (200p).
// It is possible to make £2 in the following way:
//     1×£1 + 1×50p + 2×20p + 1×5p + 1×2p + 3×1p
// How many different ways can £2 be made using any number of coins?

static COINS: [u32; 8] = [200, 100, 50, 20, 10, 5, 2, 1];

fn solve(amount_to_pay: u32, current_coin_id: usize) -> Result<usize, &'static str> {
    // Base case 1 (Nothing to pay for):
    if amount_to_pay == 0 {
        return Err("Nothing to pay for");
    }

    // Base case 2 (payment failure): No more coins left -> wrong path
    if current_coin_id >= COINS.len() {
        return Err("Unable pay this amount with the remaining coins");
    }

    let coin_value = COINS[current_coin_id];
    let max_usable_coins = amount_to_pay / coin_value;

    let result = (0..=max_usable_coins)
        .map(|num_used_coins| {
            let paid_amount = num_used_coins * coin_value;
            let remaining_amount = amount_to_pay - paid_amount;
            if remaining_amount == 0 {
                // Base case 3 (paid)
                1
            } else {
                // pay the remaining with smaller coins
                solve(remaining_amount, current_coin_id + 1).unwrap_or(0)
            }
        })
        .sum();

    Ok(result)
}

fn main() {
    println!("{}", solve(200, 0).unwrap());
}

#[test]
fn pay_0() {
    assert!(solve(0, 0).is_err())
}

#[test]
fn pay_1() {
    assert_eq!(solve(1, 0).unwrap(), 1);
}

#[test]
fn pay_2() {
    assert_eq!(solve(2, 0).unwrap(), 2);
}

#[test]
fn pay_3() {
    assert_eq!(solve(3, 0).unwrap(), 2);
}

#[test]
fn pay_4() {
    assert_eq!(solve(4, 0).unwrap(), 3);
}

#[test]
fn pay_5() {
    assert_eq!(solve(5, 0).unwrap(), 4);
}

#[test]
fn pay_200() {
    assert_eq!(solve(200, 0).unwrap() % 1000, 682)
}
