
// Make it work
use std::mem::size_of_val;
fn main() {
    let c1 = 'a';
    assert_eq!(size_of_val(&c1),4);

    let c2 = '中';
    assert_eq!(size_of_val(&c2),4);

    println!("Success!");
}



// Make it work
fn main() {
    let c1 = '中';
    print_char(c1);
}

fn print_char(c : char) {
    println!("{}", c);
}



// Make println! work
fn main() {
    let _f: bool = false;

    let t = false;
    if !t {
        println!("Success!");
    }
}



// Make it work
fn main() {
    let f = true;
    let t = true || false;
    assert_eq!(t, f);

    println!("Success!");
}


fn main() {
    let _v: () = ();

    let v = (2, 3);
    assert_eq!(v, implicitly_ret_unit()); // This will now work

    println!("Success!");
}

fn implicitly_ret_unit() -> (i32, i32) { // Fix the return type to match the tuple
    println!("I will return a ()");
    (2, 3) // Return the tuple (2, 3)
}

// Don't use this one
fn explicitly_ret_unit() -> () {
    println!("I will return a ()");
}



// Modify `4` in assert to make it work
use std::mem::size_of_val;
fn main() {
    let unit: () = ();
    assert!(size_of_val(&unit) == 0);

    println!("Success!");
}