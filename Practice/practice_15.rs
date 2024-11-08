fn gray(n: u8) -> Vec<String> {
    (0..2u8.pow(n as u32))
        .map(|i| format!("{:0width$b}", i ^ (i >> 1), width = n as usize))
        .collect()
}

fn main() {
    let n = 3;
    let result = gray(n);

    println!("Gray code for {}: {:?}", n, result);
}
