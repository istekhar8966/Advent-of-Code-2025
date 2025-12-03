use std::fs;

struct Point {
    dir: String,
    point: i32,
}

#[allow(dead_code)]
fn main() {
    let mut count: i32 = 0;
    let mut possition: i32 = 50;
    // let mut directions: Vec<Point> = Vec::new();

    // Read file into String!
    let file = fs::read_to_string("input.txt").expect("Unable to read file");

    // Go through each direction and update the dial!
    for line in file.lines() {
        let (dir_str, num_str) = line.split_at(1);

        let point = Point {
            dir: dir_str.to_string(),
            point: num_str.trim().parse::<i32>().unwrap(),
        };

        // Left direction!
        if point.dir.to_string() == 'L'.to_string() {
            possition = (possition - point.point) % 100;
            if possition == 0 {
                count += 1;
            }
        // Right direction!
        } else if point.dir.to_string() == 'R'.to_string() {
            possition = (possition + point.point) % 100;
            if possition == 0 {
                count += 1;
            }
        } else {
            println!("Invalid Direction!");
        }
    }

    println!("count: {}", count);
}
