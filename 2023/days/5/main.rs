use std::collections::HashMap;

fn to_almenac_range(range: Vec<u64>) -> (u64, u64, u64) {
    (range[0], range[1], range[2])
}

#[derive(Debug)]
struct Almenac {
    source: String,
    destination: String,
    ranges: Vec<(u64, u64, u64)>,
}

impl Almenac {
    fn from_iter(lines: &mut impl Iterator<Item = String>) -> Self {
        let source_destination_line = lines.next().unwrap().replace(" map:", "");
        let (source, destination) = source_destination_line.split_once("-to-").unwrap();

        let mut ranges = Vec::new();
        while let Some(range_str) = lines.next() {
            if range_str == "" {
                break;
            }
            let range_vec = range_str
                .split(char::is_whitespace)
                .map(|number| number.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            ranges.push(to_almenac_range(range_vec));
        }

        Self {
            source: source.to_string(),
            destination: destination.to_string(),
            ranges,
        }
    }

    fn get_destination_number(&self, source_number: u64, reverse: bool) -> u64 {
        if let Some((destination_start, source_start, _)) =
            self.ranges
                .iter()
                .find(|(destination_start, source_start, range_count)| {
                    if reverse {
                        let source_end = destination_start + range_count;
                        source_number >= *destination_start && source_number < source_end
                    } else {
                        let source_end = source_start + range_count;
                        source_number >= *source_start && source_number < source_end
                    }
                })
        {
            if reverse {
                let diff = source_number - destination_start;
                source_start + diff
            } else {
                let diff = source_number - source_start;
                destination_start + diff
            }
        } else {
            source_number
        }
    }
}

fn read_input() -> (Vec<u64>, HashMap<String, Almenac>) {
    let mut lines_iter = std::io::stdin()
        .lines()
        .filter(|line| line.is_ok())
        .map(|line| line.unwrap().trim().to_string())
        .peekable();

    let seeds_line = lines_iter.next().unwrap().replace("seeds: ", "");
    let seeds = seeds_line
        .split(char::is_whitespace)
        .map(|number| number.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let mut almenacs: HashMap<String, Almenac> = HashMap::default();

    lines_iter.next();
    while let Some(_) = lines_iter.peek() {
        let almenac = Almenac::from_iter(&mut lines_iter);
        almenacs.insert(almenac.source.clone(), almenac);
    }

    (seeds, almenacs)
}

fn find_location(
    almenac_name: &String,
    number: u64,
    almenacs: &HashMap<String, Almenac>,
    reverse: bool,
) -> u64 {
    if reverse && almenac_name == "seed" || almenac_name == "location" {
        return number;
    }

    let almenac = almenacs.get(almenac_name).unwrap();
    let next_number = almenac.get_destination_number(number, reverse);
    let next_almenac = if reverse {
        almenacs
            .iter()
            .find(|(_, a)| *a.destination == *almenac_name)
            .unwrap()
            .1
            .source
            .clone()
    } else {
        almenac.destination.clone()
    };

    find_location(&next_almenac, next_number, almenacs, reverse)
}

fn part1() -> Option<u64> {
    let (seeds, almenacs) = read_input();
    let result = seeds
        .iter()
        .map(|seed| find_location(&"seed".to_string(), *seed, &almenacs, false))
        .min();

    result
}

fn part2() -> Option<u64> {
    let (seeds, almenacs) = read_input();
    let seed_ranges: Vec<_> = seeds.chunks(2).map(|range| (range[0], range[1])).collect();

    let result = (0_u64..).find(|count| {
        let seed_number = find_location(&"humidity".to_string(), *count, &almenacs, true);
        return seed_ranges.iter().any(|(number, count)| seed_number >= *number && seed_number < (number + count))
    });

    result
}

fn main() {
    println!("--- Day 5: If You Give A Seed A Fertilizer ---");
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
