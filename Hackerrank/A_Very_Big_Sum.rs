use std::io::{self, BufRead};

fn a_very_big_sum(ar: Vec<i64>) -> i64 {
    ar.iter().sum()
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();


    let _n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();


    let ar: Vec<i64> = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let result = a_very_big_sum(ar);
    println!("{}", result);
}
