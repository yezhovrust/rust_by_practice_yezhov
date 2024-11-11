
// Remove something to make it work
fn main() {
    let x: u32 = 5;
    let mut y: u32 = 5;

    y = x;

    let z = 10; // Type of z ?

    println!("Success!");
}



// Fill the blank
fn main() {
    let v: u16 = 38_u8 as u16;

    println!("Success!");
}




// Modify `assert_eq!` to make it work
fn main() {
    let x = 5;
    assert_eq!("i32".to_string(), type_of(&x));

    println!("Success!");
}

// Get the type of given variable, return a string representation of the type  , e.g "i8", "u8", "i32", "u32"
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}




// Fill the blanks to make it work
fn main() {
    assert_eq!(i8::MAX, i8::MAX);
    assert_eq!(u8::MAX, u8::MAX);

    println!("Success!");
}



// Fix errors and panics to make it work
fn main() {
    let v1 = 251_u16 + 8;
    let v2 = i16::checked_add(251, 8).unwrap();
    println!("{},{}",v1,v2);
}



// Modify `assert!` to make it work
fn main() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    assert!(v == 1597);

    println!("Success!");
}


fn main() {
    let x = 1_000.000_1_f64; // Explicitly specify f64 for x
    let y: f32 = 0.12; // f32
    let z = 0.01_f64; // f64

    assert_eq!(type_of(&x), "f64".to_string()); // Now comparing to "f64"
    println!("Success!");
}

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}



fn main() {
    assert!(0.1_f32+0.2_f32==0.3_f32);

    println!("Success!");
}


fn main() {
    let mut sum = 0;
    for i in -3..2 {
        sum += i
    }

    assert!(sum == -5);

    for c in 'a'..='z' {
        println!("{}",c);
    }
}



// Fill the blanks
use std::ops::{Range, RangeInclusive};
fn main() {
    assert_eq!((1..5), Range{ start: 1, end: 5 });
    assert_eq!((1..=5), RangeInclusive::new(1, 5));

    println!("Success!");
}




// Fill the blanks and fix the errors
fn main() {
    // Integer addition
    assert!(1u32 + 2 == 3);

    // Integer subtraction
    assert!(1i32 - 2 == -1);
    assert!(1i8 - 2 == -1);

    assert!(3 * 50 == 150);

    assert!(9.6 / 3.2 == 3.0_f32); // error ! make it work

    assert!(24 % 5 == 4);
    // Short-circuiting boolean logic
    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}