fn read_input() -> (Vec<isize>, Vec<isize>) {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in std::io::stdin().lines().filter_map(|line| line.ok()) {
        let mut split_line = line.trim().split_whitespace();
        let left_str = split_line.next().unwrap();
        let right_str = split_line.next().unwrap();
        
        left.push(left_str.parse().unwrap());
        right.push(right_str.parse().unwrap());
    }

    (left, right)
}

fn part1() -> Option<isize> {
    let (mut left, mut right) = read_input();

    left.sort();
    right.sort();

    Some(
        left.iter()
            .zip(right.iter())
            .map(|(a, b)| (a - b).abs())
            .sum(),
    )
}

fn part2() -> Option<isize> {
    let (left, right) = read_input();

    Some(
        left.iter()
            .map(|id| id * right.iter().filter(|&right_id| id == right_id).count() as isize)
            .sum(),
    )
}

fn main() {
    println!("--- Day 1: Historian Hysteria ---");
    if let Some(part) = std::env::args().skip(1).next() {
        if let Some(result) = match part.as_str() {
            "1" => part1(),
            "2" => part2(),
            _ => panic!("💥 Invalid part number: {part}"),
        } {
            println!("🎁 Result part {part}: {result}");
        }
    } else {
        if let Some(result_1) = part1() {
            println!("🎁 Result part 1: {result_1}");
        }
        if let Some(result_2) = part2() {
            println!("🎁 Result part 2: {result_2}");
        }
    }
}
