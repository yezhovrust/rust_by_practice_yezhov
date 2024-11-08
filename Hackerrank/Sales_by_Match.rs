use std::collections::HashMap;

fn sock_merchant(socks: Vec<i32>) -> i32 {
    let mut color_count = HashMap::new();

    for sock in socks {
        *color_count.entry(sock).or_insert(0) += 1;
    }

    let mut total_pairs = 0;

    for &count in color_count.values() {
        total_pairs += count / 2;
    }

    total_pairs
}

fn main() {
    let socks = vec![10, 20, 20, 10, 10, 30, 50, 10, 20, 20];  // Example array of sock colors
    let result = sock_merchant(socks);

    println!("Total pairs: {}", result);
}
