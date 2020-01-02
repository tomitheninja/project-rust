// By starting at the top of the triangle below and moving to adjacent numbers on the row below,
// the maximum total from top to bottom is 23.

// 3
// 7 4
// 2 4 6
// 8 5 9 3

// That is, 3 + 7 + 4 + 9 = 23.

#[derive(Clone)]
pub struct TriangleItem {
    value: u32,
    sum_of_children: u32,
}

impl TriangleItem {
    fn new(value: u32) -> Self {
        TriangleItem {
            value,
            sum_of_children: 0,
        }
    }

    fn sum(&self) -> u32 {
        self.value + self.sum_of_children
    }
}

pub mod triangle {

    use super::*;

    fn parse_triangle_line(triangle_line: &[u32]) -> Vec<TriangleItem> {
        triangle_line
            .iter()
            .map(|&value| TriangleItem::new(value))
            .collect()
    }

    pub fn parse_triangle(triangle: &Vec<Vec<u32>>) -> Vec<Vec<TriangleItem>> {
        let mut result: Vec<Vec<TriangleItem>> = Vec::with_capacity(triangle.capacity());
        for line_index in (0..triangle.len()).rev() {
            let parsed_line = parse_triangle_line(&triangle[line_index]);
            result.push(parsed_line);
        }
        result
    }

    pub fn calculate_sum_of_children(
        current_line: &[TriangleItem],
        children_line: &[TriangleItem],
    ) -> Vec<TriangleItem> {
        use std::cmp::min;
        // Copy current_line to result
        let mut result = Vec::with_capacity(current_line.len());
        for item in current_line {
            result.push(item.clone());
        }
        // Solve
        for i in 0..result.len() {
            // It's children are: Some(children[i]), Option(children[i + 1])
            let slice_last_index = min(children_line.len() - 1, i + 1);
            let sum_of_children = children_line[i..=slice_last_index]
                .iter()
                .max_by_key(|&child| child.sum())
                .unwrap()
                .sum();
            result[i].sum_of_children = sum_of_children;
        }
        result
    }

    pub fn get_first_line(v: &[Vec<TriangleItem>]) -> Vec<TriangleItem> {
        let first_line = &v[0];
        let mut result: Vec<TriangleItem> = Vec::with_capacity(first_line.len());
        for x in first_line {
            result.push(x.clone());
        }
        result
    }

    pub fn print_result(v: &[Vec<TriangleItem>]) {
        for line in v.iter().rev() {
            for item in line {
                print!(
                    "({:3}|{:3}|{:4}) ",
                    item.value,
                    item.sum_of_children,
                    item.sum()
                );
            }
            println!();
        }
        println!();
        println!();
    }

    pub fn solve(triangle: Vec<Vec<u32>>) -> u32 {
        let parsed_triangle = parse_triangle(&triangle);
        let mut results: Vec<Vec<TriangleItem>> = Vec::with_capacity(parsed_triangle.capacity());
        results.push(get_first_line(&parsed_triangle));

        for i in 1..parsed_triangle.len() {
            let new_result = calculate_sum_of_children(&parsed_triangle[i], &results[i - 1]);
            results.push(new_result);
        }

        results[results.len() - 1][0].sum()
    }
}
