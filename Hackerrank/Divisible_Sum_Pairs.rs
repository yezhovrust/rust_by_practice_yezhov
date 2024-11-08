fn divisible_sum_pairs(ar: Vec<i32>, k: i32) -> i32 {
    let mut count = vec![0; k as usize];
    let mut result = 0;

    for &num in &ar {
        let remainder = ((num % k) + k) % k;
        count[remainder as usize] += 1;
    }

    for i in 0..=(k / 2) {
        if i == 0 || 2 * i == k {
            result += (count[i as usize] * (count[i as usize] - 1)) / 2;
        } else {
            result += count[i as usize] * count[(k - i) as usize];
        }
    }

    result
}

fn main() {
    let ar = vec![1, 2, 3, 4, 5, 10];
    let k = 5;
    println!("{}", divisible_sum_pairs(ar, k));
}
