
fn main() {
    // Use as many approaches as you can to make it work
    let x: String = String::from("Hello world");
    let y: String = x.clone();
    println!("{}, {}",x, y);
}


// Don't modify code in main!
fn main() {
    let s1: String = String::from("Hello world");
    let s2: String = take_ownership(s1);

    println!("{}", s2);
}

// Only modify the code below!
fn take_ownership(s: String)->String {
    println!("{}", s);
    s
}



fn main() {
    let s: String = give_ownership();
    println!("{}", s);
}

// Only modify the code below!
fn give_ownership() -> String {
    let s: String= String::from("Hello world");
    // Convert String to Vec
    let _s = s.as_bytes();
    s
}


// Fix the error without removing any code
fn main() {
    let s = String::from("Hello World");

    print_str(s.clone());

    println!("{}", s);
}

fn print_str(s: String)  {
    println!("{}",s)
}


// Don't use clone ,use copy instead
fn main() {
    let x:(i32,i32,(),&str) = (1, 2, (), "hello");
    let y:(i32,i32,(),&str) = x;
    println!("{:?}, {:?}", x, y);
}



// make the necessary variable mutable
fn main() {
    let s = String::from("Hello ");

    let mut s1 = s;

    s1.push_str("World!");

    println!("Success!");
}




fn main() {
    let x = Box::new(5);

    let mut y= Box::new(1);     // update this line, don't change other lines!

    *y = 4;

    assert_eq!(*x, 5);

    println!("Success!");
}




fn main() {
    let t = (String::from("hello"), String::from("world"));

    let _s = t.0;

    // Modify this line only, don't use `_s`
    println!("{:?}", t.1);
}



fn main() {
    let t = (String::from("hello"), String::from("world"));

    // Fill the blanks
    let (s1, s2) = t.clone();

    println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
}