use std::io::{self, BufRead};

fn round_grade(grade: u32) -> u32 {
    if grade < 40 {
        return grade;
    }

    let next_multiple_of_5 = (grade / 5 + 1) * 5;
    if next_multiple_of_5 - grade < 3 {
        return next_multiple_of_5;
    }

    grade
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();


    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();


    for _ in 0..n {
        let grade: u32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
        println!("{}", round_grade(grade));
    }
}
