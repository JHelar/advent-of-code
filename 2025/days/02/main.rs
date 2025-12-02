#[derive(Debug)]
struct IDRange {
    right: i64,
    current: i64,
}

impl IDRange {
    fn from_str(str: &str) -> Option<Self> {
        if let Some((left_str, right_str)) = str.split_once("-") {
            let left = left_str.parse().unwrap();
            let right = right_str.parse().unwrap();

            Some(Self {
                right,
                current: left,
            })
        } else {
            None
        }
    }

    fn sum_invalid_ids<P>(&mut self, pred: P) -> i64
    where
        P: Fn(<IDRange as Iterator>::Item) -> bool,
    {
        self.fold(0, |sum, num| if pred(num) { sum + num } else { sum })
    }
}

impl Iterator for IDRange {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;
        if current > self.right {
            None
        } else {
            self.current += 1;
            Some(current)
        }
    }
}

fn is_repeat(num: i64) -> bool {
    let num_str = num.to_string();

    if num_str.len() % 2 > 0 {
        return false;
    }
    let (left, right) = num_str.split_at(num_str.len() / 2);

    left == right
}

fn is_sequence(num: i64) -> bool {
    let num_str = num.to_string();

    let chars: Vec<char> = num_str.chars().collect();

    for window_size in 1..chars.len() {
        if chars.len() % window_size > 0 {
            continue;
        }
        
        let mut sequence_found = true;
        for index in 0..(chars.len() - window_size) {
            let left = chars[index];
            let right = chars[index + window_size];

            if left != right {
                sequence_found = false;
                break;
            }
        }

        if sequence_found {
            return true;
        }
    }
    false
}

fn read_input() -> Vec<IDRange> {
    let line = std::io::stdin()
        .lines()
        .filter_map(|line| line.ok())
        .next()
        .unwrap();

    line.split(",").filter_map(IDRange::from_str).collect()
}

fn part1() -> Option<i64> {
    Some(
        read_input()
            .iter_mut()
            .map(|range| range.sum_invalid_ids(is_repeat))
            .sum(),
    )
}

fn part2() -> Option<i64> {
    Some(
        read_input()
            .iter_mut()
            .map(|range| range.sum_invalid_ids(is_sequence))
            .sum(),
    )
}

fn main() {
    println!("--- Day 2: Gift Shop ---");
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
