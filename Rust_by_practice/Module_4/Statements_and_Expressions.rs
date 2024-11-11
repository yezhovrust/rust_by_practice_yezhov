// Make it work with two ways
fn main() {
    let v: i32 = {
        let mut x = 1;
        x =x+ 2;
        x
    };

    assert_eq!(v, 3);

    println!("Success!");
}


fn main() {
    let v = {
        let x = 3;
        x
    };

    assert!(v == 3);

    println!("Success!");
}



fn main() {
    let s = sum(1 , 2);
    assert_eq!(s, 3);

    println!("Success!");
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}