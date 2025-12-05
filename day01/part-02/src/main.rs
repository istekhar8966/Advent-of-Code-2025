use std::{fs::read_to_string, io};

fn main() -> io::Result<()> {
    let file = read_to_string("input")?;

    let mut count = 0;
    let mut pos: i32 = 50;
    let max = 100;

    for line in file.lines() {
        let (dir, dis_str) = line.split_at(1);
        let dis: i32 = dis_str.parse().unwrap();
        let dir: char = dir.chars().next().unwrap();

        for _ in 0..dis {
            if dir == 'L' {
                pos = (pos - 1).rem_euclid(max);
            } else {
                pos = (pos + 1).rem_euclid(max);
            }
            if pos == 0 {
                count += 1;
            }
        }
    }

    println!("{}", count);
    Ok(())
}
