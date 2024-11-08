fn bon_appetit(bill: Vec<i32>, k: usize, b: i32) {
    let total: i32 = bill.iter().enumerate()
        .filter(|(i, _)| *i != k)
        .map(|(_, &x)| x)
        .sum();

    let correct_share = total / 2;

    if b == correct_share {
        println!("Bon Appetit");
    } else {
        println!("{}", b - correct_share);
    }
}

fn main() {
    let bill = vec![3, 10, 2, 9];
    let k = 1;
    let b = 12;

    bon_appetit(bill, k, b);
}
