// Quadratic primes
// Problem 28

// Starting with the number 1 and moving to the right in a clockwise direction a 5 by 5 spiral is formed as follows:

// 21 22 23 24 25
// 20  7  8  9 10
// 19  6  1  2 11
// 18  5  4  3 12
// 17 16 15 14 13

// It can be verified that the sum of the numbers on the diagonals is 101.
// What is the sum of the numbers on the diagonals in a 1001 by 1001 spiral formed in the same way?

enum Direction {
    INITIAL,
    RIGHT,
    DOWN,
    LEFT,
    UP,
}

impl Direction {
    fn next(&self) -> Self {
        match self {
            Direction::LEFT => Direction::UP,
            Direction::UP | Direction::INITIAL => Direction::RIGHT,
            Direction::RIGHT => Direction::DOWN,
            Direction::DOWN => Direction::LEFT,
        }
    }
}

struct Point(usize, usize);

impl Point {
    fn next(&self, dir: &Direction) -> Point {
        Point(
            match dir {
                Direction::LEFT => self.0 - 1,
                Direction::RIGHT => self.0 + 1,
                _ => self.0,
            },
            match dir {
                Direction::UP => self.1 - 1,
                Direction::DOWN => self.1 + 1,
                _ => self.1,
            },
        )
    }
}

fn gen_spiral(size: usize) -> Vec<Vec<u32>> {
    assert_eq!(size % 2, 1, "size must be odd");
    let mut board = vec![vec![0 as u32; size]; size];
    let initial_position = (size as f64 / 2.0).floor() as usize;
    let mut value: u32 = 1;
    let mut required_steps = 1;
    let mut current_dir = Direction::RIGHT;
    let mut position = Point(initial_position, initial_position);
    loop {
        for _ in 0..2 {
            let mut remaining_steps = required_steps;
            while remaining_steps > 0 {
                if position.0 == size || position.1 == size {
                    return board;
                }
                board[position.1][position.0] = value;
                value += 1;
                remaining_steps -= 1;
                position = position.next(&current_dir);
            }
            current_dir = current_dir.next();
        }
        required_steps += 1;
    }
}

fn cross_sum(spiral: &Vec<Vec<u32>>) -> usize {
    let half_len = spiral.len() / 2;
    let mut sum = spiral[half_len][half_len];
    for p in 0..half_len {
        let p1 = p;
        let p2 = spiral.len() - 1 - p;
        sum += spiral[p1][p1] + spiral[p1][p2] + spiral[p2][p2] + spiral[p2][p1];
    }
    sum as usize
}

fn compute(size: usize) -> usize {
    let spiral = gen_spiral(size);
    cross_sum(&spiral)
}

fn main() {
    println!("{}", compute(1001));
}

#[test]
fn compute_5() {
    assert_eq!(compute(5), 101);
}

#[test]
fn compute_1001() {
    assert_eq!(compute(1001) % 10000, 1001);
}
