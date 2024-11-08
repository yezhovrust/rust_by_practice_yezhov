use std::io::{self, BufRead};

fn mini_max_sum(arr: Vec<u64>) {
    let mut arr = arr.clone();
    arr.sort();

    let min_sum: u64 = arr.iter().take(4).sum();
    let max_sum: u64 = arr.iter().skip(1).sum();

    println!("{} {}", min_sum, max_sum);
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let arr: Vec<u64> = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    mini_max_sum(arr);
}
