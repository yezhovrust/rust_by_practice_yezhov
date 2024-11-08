use rand::Rng;

fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(10..100)).collect()
}

fn min_adjacent_sum(data: &[i32]) -> (i32, usize, usize) {
    let mut min_sum = i32::MAX;
    let mut min_pair = (0, 0);

    for i in 0..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
            min_pair = (i, i + 1);
        }
    }

    (min_sum, min_pair.0, min_pair.1)
}

fn print_result(data: Vec<i32>, min_sum: i32, idx1: usize, idx2: usize) {

    println!("indexes: {}", (0..data.len()).map(|i| format!("{:<3}", i)).collect::<String>());

    println!("data:   {}", data.iter().map(|x| format!("{:<3}", x)).collect::<String>());

    println!("indexes:{}", (0..data.len())
        .map(|i| if i == idx1 || i == idx2 { "\\__" } else { "   " })
        .collect::<String>()
    );

    println!("min adjacent sum={}+{}={} at indexes:{},{}", data[idx1], data[idx2], min_sum, idx1, idx2);
}

fn main() {
    let n = 20;
    let data = gen_random_vector(n);
    let (min_sum, idx1, idx2) = min_adjacent_sum(&data);
    print_result(data, min_sum, idx1, idx2);і
}
