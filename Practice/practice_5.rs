fn main() {
    const WIDTH: usize = 26;
    const HEIGHT: usize = 13;


    for i in 0..HEIGHT {
        for j in 0..WIDTH {

            if i == 0 || i == HEIGHT - 1 {
                print!("*");
            }

            else if j == 0 || j == WIDTH - 1 {
                print!("*");
            }

            else if j == i || j == WIDTH - i - 1 {
                print!("*");
            }
            else {
                print!(" ");
            }
        }
        println!();
    }
}
