fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    let a = 24;
    let b = 60;
    println!("Найбільший спільний дільник: {}", gcd(a, b));
}
