fn compare_triplets(a: [i32; 3], b: [i32; 3]) -> (i32, i32) {
    let mut alice_score = 0;
    let mut bob_score = 0;

    for i in 0..3 {
        if a[i] > b[i] {
            alice_score += 1;
        } else if a[i] < b[i] {
            bob_score += 1;
        }
    }

    (alice_score, bob_score)
}

fn main() {
    let a = [1, 2, 3]; //Alice
    let b = [3, 2, 1]; // Bob

    let result = compare_triplets(a, b);
    println!("{:?}", result);
}
