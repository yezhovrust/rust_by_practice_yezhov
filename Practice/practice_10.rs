fn rotate(s: String, n: isize) -> String {
    let len = s.len();


    if len == 0 {
        return s;
    }


    let n = n.rem_euclid(len as isize) as usize;


    let (first, second) = s.split_at(len - n);
    format!("{}{}", second, first)
}

fn main() {
    let s = "abcdefgh".to_string();
    let rotated = rotate(s.clone(), 2);
    println!("Поворот {}", rotated);
}
