use std::cmp;

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: i32, b: i32) -> i32 {
    a * b / gcd(a, b)
}

fn find_numbers_between_arrays(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
    let lcm_val = arr1.iter().fold(1, |acc, &x| lcm(acc, x));

    let gcd_val = arr2.iter().fold(arr2[0], |acc, &x| gcd(acc, x));

    let mut count = 0;
    let mut x = lcm_val;

    while x <= gcd_val {
        if gcd_val % x == 0 {
            count += 1;
        }
        x += lcm_val;
    }

    count
}

fn main() {
    let arr1 = vec![2, 4];
    let arr2 = vec![16, 32, 96];

    let result = find_numbers_between_arrays(arr1, arr2);

    println!("Number of valid integers: {}", result);
}
