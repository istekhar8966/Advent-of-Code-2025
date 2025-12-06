fn is_repeated(num: i64) -> bool {
    let s = num.to_string();

    if s.len() % 2 != 0 {
        return false; // cannot split evenly
    }

    let mid = s.len() / 2;
    let (left, right) = s.split_at(mid);

    left == right
}

fn main() {
    let file = std::fs::read_to_string("input").unwrap();
    let file = file.trim().split(",");
    let mut sum = 0;

    for line in file {
        let (start, end) = line.split_once("-").unwrap();

        let start = start.parse::<i64>().unwrap();
        let end = end.parse::<i64>().unwrap();

        for num in start..=end {
            if is_repeated(num) {
                sum += num;
            }
        }
    }
    println!("{}", sum);
}
