fn draw_tree(triangles: usize) {

    (1..=triangles).for_each(|i| {

        let height = i + 1;

        (0..height).for_each(|j| {
            let spaces = triangles + height - j - 1;
            let stars = 2 * j + 1;
            println!("{:width$}{}", "", "*".repeat(stars), width = spaces);
        });
    });


    let trunk_width = triangles;
    let trunk_height = 2;
    (0..trunk_height).for_each(|_| {
        let spaces = " ".repeat(triangles - 1);
        println!("{}*", spaces);
    });
}

fn main() {

    let triangles = 5;
    draw_tree(triangles);
}
