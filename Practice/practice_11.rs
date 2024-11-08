fn is_palindrome(x: u32) -> bool {
    let s = x.to_string();
    s == s.chars().rev().collect::<String>()
}

fn main() {
    let num = 12321;
    println!("Число {} є паліндромом: {}", num, is_palindrome(num));

    let num2 = 12345;
    println!("Число {} є паліндромом: {}", num2, is_palindrome(num2));
}