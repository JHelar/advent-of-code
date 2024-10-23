use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    fmt::Display,
};

#[derive(Debug, Clone)]
struct Checksum {
    source: String,
    map: HashMap<char, usize>,
}

impl Display for Checksum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.source)
    }
}

impl Ord for Checksum {
    fn cmp(&self, other: &Self) -> Ordering {
        let diff_size = self.source.len() - self.common(other).len();
        
        if diff_size == 1 {
            Ordering::Equal
        } else if diff_size > 1 {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }
}

impl PartialOrd for Checksum {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Checksum {
    fn eq(&self, other: &Self) -> bool {
        self.source == other.source
    }
}

impl Eq for Checksum {}

impl Checksum {
    fn from_string(str: &str) -> Self {
        let unique: HashSet<char> = str.chars().collect();
        Self {
            source: str.to_string(),
            map: unique
                .iter()
                .map(|&c| (c, str.matches(c).count()))
                .collect(),
        }
    }

    fn common(&self, other: &Checksum) -> String {
        self.source
            .chars()
            .zip(other.source.chars())
            .filter(|&(a, b)| a == b)
            .map(|(a, _)| a)
            .collect::<String>()
    }

    fn get_value(&self) -> (u32, u32) {
        let two = if self.map.values().any(|&v| v == 2) {
            1
        } else {
            0
        };
        let three = if self.map.values().any(|&v| v == 3) {
            1
        } else {
            0
        };
        (two, three)
    }
}

fn read_input() -> Vec<Checksum> {
    std::io::stdin()
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| Checksum::from_string(line.trim()))
        .collect()
}

fn part1() -> Option<u32> {
    let checksums = read_input();
    let (two, three) = checksums.iter().fold((0, 0), |(two, three), checksum| {
        let (c_two, c_three) = checksum.get_value();
        (c_two + two, c_three + three)
    });
    Some(two * three)
}

fn part2() -> Option<u32> {
    let checksums = read_input();
    'outer: for a in checksums.clone().iter() {
      for b in checksums.clone().iter().rev() {
        if matches!(a.cmp(b), Ordering::Equal) {
          println!("🎁 Result part 2: {}", a.common(b));
          break 'outer;
        }
      }
    }
    None
}

fn main() {
    println!("--- Day 2: Inventory Management System ---");
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
