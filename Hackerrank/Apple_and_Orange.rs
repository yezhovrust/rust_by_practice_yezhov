use std::io::{self, BufRead};

fn count_fruits_on_house(s: i32, t: i32, a: i32, b: i32, apples: Vec<i32>, oranges: Vec<i32>) -> (i32, i32) {
    let apples_on_house = apples.iter().filter(|&&d| a + d >= s && a + d <= t).count() as i32;
    let oranges_on_house = oranges.iter().filter(|&&d| b + d >= s && b + d <= t).count() as i32;

    (apples_on_house, oranges_on_house)
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read s, t, a, b
    let first_line = lines.next().unwrap().unwrap();
    let (s, t, a, b): (i32, i32, i32, i32) = {
        let nums: Vec<i32> = first_line.split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        (nums[0], nums[1], nums[2], nums[3])
    };

    let _ = lines.next();

    let apples: Vec<i32> = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|d| d.parse().unwrap())
        .collect();
    let oranges: Vec<i32> = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|d| d.parse().unwrap())
        .collect();

    let (apples_on_house, oranges_on_house) = count_fruits_on_house(s, t, a, b, apples, oranges);

    println!("{}", apples_on_house);
    println!("{}", oranges_on_house);
}
