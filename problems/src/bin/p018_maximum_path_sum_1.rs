// By starting at the top of the triangle below and moving to adjacent numbers on the row below,
// the maximum total from top to bottom is 23.

// 3
// 7 4
// 2 4 6
// 8 5 9 3

// That is, 3 + 7 + 4 + 9 = 23.

// Find the maximum total from top to bottom of the triangle below:

// NOTE: As there are only 16384 routes, it is possible to solve this problem by trying every route.
// However, Problem 67, is the same challenge with a triangle containing one-hundred rows; it cannot be solved by brute force, and requires a clever method! ;o)

extern crate path_sum_triangle;

use path_sum_triangle::triangle::solve;

fn main() {
    println!("p018: {}", solve(get_large_triangle()));
}

#[cfg(test)]
mod p018 {
    use super::*;
    use path_sum_triangle::triangle::solve;

    #[test]
    fn small_triangle() {
        assert_eq!(23, solve(get_small_triangle()));
    }

    #[test]
    fn large_triangle() {
        assert_eq!(74, solve(get_large_triangle()) % 100);
    }

    fn get_small_triangle() -> Vec<Vec<u32>> {
        vec![vec![3], vec![7, 4], vec![2, 4, 6], vec![8, 5, 9, 3]]
    }
}

#[allow(clippy::all)]
fn get_large_triangle() -> Vec<Vec<u32>> {
    vec![
        vec![75],
        vec![95, 64],
        vec![17, 47, 82],
        vec![18, 35, 87, 10],
        vec![20, 04, 82, 47, 65],
        vec![19, 01, 23, 75, 03, 34],
        vec![88, 02, 77, 73, 07, 63, 67],
        vec![99, 65, 04, 28, 06, 16, 70, 92],
        vec![41, 41, 26, 56, 83, 40, 80, 70, 33],
        vec![41, 48, 72, 33, 47, 32, 37, 16, 94, 29],
        vec![53, 71, 44, 65, 25, 43, 91, 52, 97, 51, 14],
        vec![70, 11, 33, 28, 77, 73, 17, 78, 39, 68, 17, 57],
        vec![91, 71, 52, 38, 17, 14, 91, 43, 58, 50, 27, 29, 48],
        vec![63, 66, 04, 68, 89, 53, 67, 30, 73, 16, 69, 87, 40, 31],
        vec![04, 62, 98, 27, 23, 09, 70, 98, 73, 93, 38, 53, 60, 04, 23],
    ]
}
