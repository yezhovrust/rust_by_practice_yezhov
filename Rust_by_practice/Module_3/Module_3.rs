
// Fix the error below with least amount of modification to the code
fn test_1() {
    let x: i32=5; // Uninitialized but used, ERROR !
    let y: i32; // Uninitialized but also unused, only a Warning !

    assert_eq!(x, 5);
    println!("Success!");
}



// Fill the blanks in the code to make it compile
fn main() {
    let mut x: i32 = 1;
    x += 2;

    assert_eq!(x, 3);
    println!("Success!");
}



// Fix the error below with least amount of modification
fn main() {
    let x: i32 = 10;
    {
        let y: i32 = 5;
        println!("The value of x is {} and value of y is {}", x, y);
    }
    println!("The value of x is {}", x);
}




fn main() {
    let _x = 1;
}

// Warning: unused variable: `x`



// Fix the error below with least amount of modification
fn main() {
    let (mut x, y) = (1, 2);
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Success!");
}



fn main() {
    let (mut x,mut y);
    (x, y) = (3, 4);
    [x, y] = [1, 2];
    // Fill the blank to make the code work
    assert_eq!([x,y], [1,2]);

    println!("Success!");
}


fn main() {
    let x = define_x();
    println!("{}, world", x);
}
fn define_x() -> String {
    let x = "hello".to_string(); // Convert the string literal to String
    x
}



// Only modify `assert_eq!` to make the `println!` work(print `42` in terminal)
fn main() {
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12);
    }

    assert_eq!(x, 5);

    let x = 42;
    println!("{}", x); // Prints "42".
}



// Remove a line in the code to make it compile
fn main() {
    let mut x: i32 = 1;
    x = 7;
    // Shadowing and re-binding
    x += 3;


    let y = 4;
    // Shadowing
    let y = "I can also be bound to text!";

    println!("Success!");
}