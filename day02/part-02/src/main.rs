fn is_repeated(num: i64) -> bool {
    let num = num.to_string();
    let doubled = format!("{}{}", num, num);
    doubled[1..doubled.len() - 1].contains(&num)
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
                println!("{}", num);
                sum += num;
            }
        }
    }
    println!("{}", sum);
}
