use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct IDRange {
    start: usize,
    end: usize,
}

impl IDRange {
    fn from_str(str: &str) -> Self {
        let (left, right) = str.split_once("-").unwrap();
        Self {
            start: left.parse().unwrap(),
            end: right.parse().unwrap(),
        }
    }

    fn contains(&self, id: usize) -> bool {
        self.start <= id && id <= self.end
    }

    fn count_valid(&self) -> usize {
        self.end - self.start + 1
    }

    fn try_merge(a: &Self, b: &Self) -> Option<Self> {
        if a == b {
            Some(a.clone())
        } else if a.end >= b.start && a.end <= b.end {
            Some(Self {
                start: a.start,
                end: b.end,
            })
        } else if a.end >= b.start {
            Some(Self {
                start: a.start,
                end: a.end,
            })
        } else {
            None
        }
    }
}

fn read_input() -> (Vec<IDRange>, Vec<usize>) {
    let mut ranges: Vec<IDRange> = Vec::default();
    let mut ids: Vec<usize> = Vec::default();

    let mut read_ids = true;
    for line in std::io::stdin().lines().filter_map(|line| line.ok()) {
        if line.is_empty() {
            if read_ids {
                read_ids = false;
                continue;
            } else {
                break;
            }
        }

        if read_ids {
            ranges.push(IDRange::from_str(&line))
        } else {
            ids.push(line.parse().unwrap());
        }
    }

    return (ranges, ids);
}

fn part1() -> Option<usize> {
    let (ranges, ids) = read_input();
    let valid_ids = ids
        .iter()
        .filter(|id| ranges.iter().any(|range| range.contains(**id)))
        .count();
    Some(valid_ids)
}

fn part2() -> Option<usize> {
    let (mut ranges, _) = read_input();

    ranges.sort_by(|a, b| a.start.cmp(&b.start));

    let mut reduced_ranges = Vec::default();
    let mut look_at_index = 0;
    while look_at_index < ranges.len() {
        let mut look_at = ranges[look_at_index].clone();

        for b_index in (look_at_index + 1)..ranges.len() {
            let range = &ranges[b_index];

            if let Some(merged_range) = IDRange::try_merge(&look_at, range) {
                look_at = merged_range;
                look_at_index += 1;
            } else {
                break;
            }
        }
        look_at_index += 1;
        reduced_ranges.push(look_at);
    }

    let valid_ids: usize = reduced_ranges.iter().map(|range| range.count_valid()).sum();
    Some(valid_ids)
}

fn main() {
    println!("--- Day 5: Cafeteria ---");
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
