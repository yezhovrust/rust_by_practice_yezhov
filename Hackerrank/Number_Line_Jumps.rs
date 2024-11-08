fn kangaroo_meet(x1: i32, v1: i32, x2: i32, v2: i32) -> String {
    if v1 == v2 {
        if x1 == x2 {
            return String::from("YES");
        } else {
            return String::from("NO");
        }
    }

    if (x2 - x1) % (v1 - v2) == 0 {
        let n = (x2 - x1) / (v1 - v2);
        if n > 0 {
            return String::from("YES");
        }
    }

    String::from("NO")
}

fn main() {
    let result1 = kangaroo_meet(0, 3, 4, 2);
    println!("{}", result1); // Output: YES

    let result2 = kangaroo_meet(0, 2, 5, 3);
    println!("{}", result2); // Output: NO
}
