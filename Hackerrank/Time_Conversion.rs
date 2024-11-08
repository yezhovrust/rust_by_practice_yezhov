use std::io;

fn convert_to_24_hour_format(time: &str) -> String {
    let (time_part, period) = time.split_at(8);
    let hour: u32 = time_part[0..2].parse().unwrap();

    let converted_hour = match period {
        "AM" => {
            if hour == 12 { 0 } else { hour }
        },
        "PM" => {
            if hour == 12 { 12 } else { hour + 12 }
        },
        _ => hour,
    };

    format!("{:02}:{:02}:{:02}", converted_hour, &time_part[3..5], &time_part[6..8])
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let time = input.trim();

    let result = convert_to_24_hour_format(time);
    println!("{}", result);
}
