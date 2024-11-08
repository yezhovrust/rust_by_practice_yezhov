use std::io::{self, BufRead};

fn diagonal_difference(matrix: Vec<Vec<i32>>) -> i32 {
    let n = matrix.len();
    let mut primary_diagonal_sum = 0;
    let mut secondary_diagonal_sum = 0;

    for i in 0..n {
        primary_diagonal_sum += matrix[i][i];
        secondary_diagonal_sum += matrix[i][n - 1 - i];
    }

    (primary_diagonal_sum - secondary_diagonal_sum).abs()
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut matrix = Vec::new();

    for _ in 0..n {
        let row: Vec<i32> = lines.next().unwrap().unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        matrix.push(row);
    }

    let result = diagonal_difference(matrix);
    println!("{}", result);
}
