use std::io::{self, BufRead};

fn plus_minus(arr: Vec<i32>) {
    let total_count = arr.len() as f64;

    let mut positive_count = 0;
    let mut negative_count = 0;
    let mut zero_count = 0;

    for &num in &arr {
        if num > 0 {
            positive_count += 1;
        } else if num < 0 {
            negative_count += 1;
        } else {
            zero_count += 1;
        }
    }

    let positive_ratio = positive_count as f64 / total_count;
    let negative_ratio = negative_count as f64 / total_count;
    let zero_ratio = zero_count as f64 / total_count;

    println!("{:.6}", positive_ratio);
    println!("{:.6}", negative_ratio);
    println!("{:.6}", zero_ratio);
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();


    let _n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();


    let arr: Vec<i32> = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    plus_minus(arr);
}
