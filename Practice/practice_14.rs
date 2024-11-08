use std::cmp::{max, min};


#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}


#[derive(Debug, Clone)]
struct Rectangle {
    a: Point,
    b: Point,
}


fn overlap_area(r1: &Rectangle, r2: &Rectangle) -> i32 {
    let x_overlap = max(0, min(r1.b.x, r2.b.x) - max(r1.a.x, r2.a.x));
    let y_overlap = max(0, min(r1.a.y, r2.a.y) - max(r1.b.y, r2.b.y));

    x_overlap * y_overlap
}


fn area_occupied(xs: &Vec<Rectangle>) -> i32 {
    let mut total_area = 0;
    let mut overlap_area_total = 0;


    for r in xs.iter() {
        let width = (r.b.x - r.a.x).abs();
        let height = (r.a.y - r.b.y).abs();
        total_area += width * height;
    }


    for i in 0..xs.len() {
        for j in i+1..xs.len() {
            overlap_area_total += overlap_area(&xs[i], &xs[j]);
        }
    }

    total_area - overlap_area_total
}


fn test_data() -> Vec<Rectangle> {
    vec![
        Rectangle {
            a: Point { x: 2, y: 9 },
            b: Point { x: 5, y: 3 },
        },
        Rectangle {
            a: Point { x: 1, y: 8 },
            b: Point { x: 11, y: 6 },
        },
        Rectangle {
            a: Point { x: 9, y: 10 },
            b: Point { x: 13, y: 2 },
        },
    ]
}


fn area_occupied_test() {
    let data = test_data();
    let occupied = area_occupied(&data);
    assert_eq!(occupied, 60);
}

fn main() {

    area_occupied_test();


    let data = test_data();


    let occupied = area_occupied(&data);
    println!("Фактична зайнята площа: {}", occupied);
}
