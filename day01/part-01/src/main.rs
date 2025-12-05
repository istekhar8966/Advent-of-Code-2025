use std::{arch::x86_64::_SIDD_CMP_EQUAL_ANY, fs};

struct Point<'a> {
    dir: &'a str,
    point: i32,
}

fn main() {
    let mut count: i32 = 0;
    let mut pos: i32 = 50;
    let max: i32 = 100; // The maximum value of the dial!

    // Read file into String!
    let file = fs::read_to_string("input.txt").expect("Unable to read file");

    // Go through each direction and update the dial!
    for line in file.lines() {
        let (dir_str, num_str) = line.split_at(1);

        let point = Point {
            dir: dir_str,
            point: num_str.trim().parse::<i32>().unwrap(),
        };

        // Left direction!
        if point.dir == "L" {
            pos = (pos - point.point).rem_euclid(max);
            if pos == 0 {
                count += 1;
            }
            // Right direction!
        } else if point.dir == "R" {
            pos = (pos + point.point).rem_euclid(max);
            if pos == 0 {
                count += 1;
            }
        } else {
            println!("Invalid Direction!");
        }
    }

    println!("count: {}", count);
}
