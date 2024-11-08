use std::io::{self, BufRead};

fn simple_array_sum(ar: Vec<i32>) -> i32 {
    ar.iter().sum()
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    println!("Введіть розмір масиву");
    let _n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    println! ("Введіть елементи масиву в кількості {}:", _n);
    let ar: Vec<i32> = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let result = simple_array_sum(ar);
    println!("{}", result);
}
