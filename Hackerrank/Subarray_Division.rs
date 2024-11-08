fn birthday(s: Vec<i32>, d: i32, m: usize) -> i32 {
    let mut count = 0;

    let mut current_sum: i32 = s.iter().take(m).sum();

    if current_sum == d {
        count += 1;
    }

    for i in m..s.len() {
        current_sum = current_sum + s[i] - s[i - m];

        if current_sum == d {
            count += 1;
        }
    }

    count
}

fn main() {
    let s = vec![1, 2, 1, 3, 2];
    let d = 3;
    let m = 2;

    println!("{}", birthday(s, d, m));
}
