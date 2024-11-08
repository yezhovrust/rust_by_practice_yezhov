fn invert_the_case(s: String) -> String {
    s.chars()
        .map(|c| {
            if c.is_lowercase() {
                c.to_uppercase().next().unwrap()
            } else {
                c.to_lowercase().next().unwrap()
            }
        })
        .collect()
}

fn main() {
    let input = "Hello World!";
    let result = invert_the_case(input.to_string());
    println!("Перетворений рядок: {}", result);
}
