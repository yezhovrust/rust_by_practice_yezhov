fn record_breaks(scores: Vec<i32>) -> (i32, i32) {
    let mut max_points = scores[0];
    let mut min_points = scores[0];

    let mut max_breaks = 0;
    let mut min_breaks = 0;

    for &score in scores.iter().skip(1) {
        if score > max_points {
            max_points = score;
            max_breaks += 1;
        }

        if score < min_points {
            min_points = score;
            min_breaks += 1;
        }
    }

    (max_breaks, min_breaks)
}

fn main() {
    let scores = vec![10, 5, 20, 20, 4, 10, 25, 3, 30];
    let (max_breaks, min_breaks) = record_breaks(scores);

    println!("Most points record breaks: {}", max_breaks);
    println!("Least points record breaks: {}", min_breaks);
}
