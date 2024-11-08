use std::collections::HashMap;

fn migratory_birds(arr: Vec<i32>) -> i32 {
    let mut frequency = HashMap::new();

    for &bird in &arr {
        *frequency.entry(bird).or_insert(0) += 1;
    }

    let mut max_count = 0;
    let mut min_bird_id = i32::MAX;

    for (&bird, &count) in &frequency {
        if count > max_count || (count == max_count && bird < min_bird_id) {
            max_count = count;
            min_bird_id = bird;
        }
    }

    min_bird_id
}

fn main() {
    let bird_sightings = vec![1, 1, 2, 2, 3, 3, 3, 4];
    println!("{}", migratory_birds(bird_sightings));
}
