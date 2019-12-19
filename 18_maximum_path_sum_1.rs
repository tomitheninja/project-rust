// By starting at the top of the triangle below and moving to adjacent numbers on the row below, the maximum total from top to bottom is 23.

// 3
// 7 4
// 2 4 6
// 8 5 9 3

// That is, 3 + 7 + 4 + 9 = 23.

// Find the maximum total from top to bottom of the triangle below:

// NOTE: As there are only 16384 routes, it is possible to solve this problem by trying every route. However, Problem 67, is the same challenge with a triangle containing one-hundred rows; it cannot be solved by brute force, and requires a clever method! ;o)

fn get_triangle () -> Vec<Vec<i32>> {
    vec![
        vec![75],
        vec![95,64],
        vec![17,47,82],
        vec![18,35,87,10],
        vec![20,04,82,47,65],
        vec![19,01,23,75,03,34],
        vec![88,02,77,73,07,63,67],
        vec![99,65,04,28,06,16,70,92],
        vec![41,41,26,56,83,40,80,70,33],
        vec![41,48,72,33,47,32,37,16,94,29],
        vec![53,71,44,65,25,43,91,52,97,51,14],
        vec![70,11,33,28,77,73,17,78,39,68,17,57],
        vec![91,71,52,38,17,14,91,43,58,50,27,29,48],
        vec![63,66,04,68,89,53,67,30,73,16,69,87,40,31],
        vec![04,62,98,27,23,09,70,98,73,93,38,53,60,04,23],
        // vec![3],
        // vec![7, 4],
        // vec![2, 4, 6],
        // vec![8, 5, 9, 3],
    ]
}

struct TriangleItem {
    value: i32,
    sum_of_children: i32
}

impl TriangleItem {

    fn new (value: i32) -> TriangleItem { TriangleItem { value, sum_of_children: 0 } }

    fn sum (&self) -> i32 { self.value + self.sum_of_children }

    fn copy(&self) -> TriangleItem {
        TriangleItem {
            value: self.value,
            sum_of_children: self.sum_of_children
        }
    }
}

fn parse_triangle_line (triangle_line: &Vec<i32>) -> Vec<TriangleItem> {
    triangle_line.iter()
        .map(|&value| TriangleItem::new(value) )
        .collect()
}

fn parse_triangle (triangle: &Vec<Vec<i32>>) -> Vec<Vec<TriangleItem>> {
    let mut result: Vec<Vec<TriangleItem>> = Vec::with_capacity(triangle.capacity());

    for line_index in (0..triangle.len()).rev() {
        let parsed_line = parse_triangle_line(&triangle[line_index]);
        result.push(parsed_line);
    }

    result
}

fn calculate_sum_of_children (current_line: &Vec<TriangleItem>, children_line: &Vec<TriangleItem>) -> Vec<TriangleItem> {
    // Copy current_line to result
    let mut result = Vec::with_capacity(current_line.len());
    for i in 0..current_line.len() {
        result.push(current_line[i].copy());
    }
    // Solve
    for i in 0..result.len() {
        // It's children are : children[i], and if exists children[i + 1]
        let slice_last_index = std::cmp::min(children_line.len() - 1, i + 1);
        
        let maybe_best_child = children_line[i..=slice_last_index].iter()
            .max_by_key(|&child| child.sum());
        
        let best_child = match maybe_best_child {
            None => panic!("No children??"),
            Some(x) => x,
        };

        result[i].sum_of_children = best_child.sum();
    }
    result
}

fn get_first_line (v: &Vec<Vec<TriangleItem>>) -> Vec<TriangleItem> {
    let first_line = &v[0];
    let mut result: Vec<TriangleItem> = Vec::with_capacity(first_line.len());
    for x in first_line {
        result.push(x.copy());
    }
    result
}

// fn print_result (v: &Vec<Vec<TriangleItem>>) {
//     for line in v.iter().rev() {
//         for item in line {
//             print!("({:3}|{:3}|{:4}) ", item.value, item.sum_of_children, item.sum());
//         }
//         print!("\n");
//     }
//     print!("\n\n");
// }

fn main () {
    let parsed_triangle = parse_triangle(&get_triangle());
    let mut results: Vec<Vec<TriangleItem>> = Vec::with_capacity(parsed_triangle.capacity());
    results.push(get_first_line(&parsed_triangle));

    for i in 1..parsed_triangle.len() {
        let new_result = calculate_sum_of_children(&parsed_triangle[i], &results[i - 1]);
        results.push(new_result);
    }

    let solution = &results[results.len() - 1][0];
    println!("{}", solution.sum());

}
