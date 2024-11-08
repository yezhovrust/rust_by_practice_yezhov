use std::io::{self, BufRead};

fn birthday_cake_candles(candles: Vec<u32>) -> u32 {
    let max_height = *candles.iter().max().unwrap();
    candles.iter().filter(|&&height| height == max_height).count() as u32
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let _n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    let candles: Vec<u32> = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let result = birthday_cake_candles(candles);
    println!("{}", result);
}
