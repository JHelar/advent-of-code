use std::collections::HashMap;

fn read_input() -> Vec<(String, Vec<u64>)> {
    std::io::stdin()
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            let (hot_springs, records) = line.trim().split_once(char::is_whitespace).unwrap();
            (
                hot_springs.to_string(),
                records
                    .split(",")
                    .map(|c| c.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>(),
            )
        })
        .collect()
}

fn search(spring_width_pointer: usize, spring_widths: &Vec<u64>, springs: String, mem: &mut HashMap<(usize, String), u64>) -> u64 {
    if let Some(cached) = mem.get(&(spring_width_pointer, springs.clone())) {
        return *cached;
    }

    let total_width = spring_widths[spring_width_pointer..].iter().map(|num| *num).sum::<u64>();
    let springs_remaining = springs.clone().chars().map(|c| c == '#' || c == '?').count();
    if total_width > springs_remaining as u64 {
        return 0;
    }
    if spring_width_pointer >= spring_widths.len() {
        return if springs.contains('#') { 0 } else { 1 };
    }
    let normalized_spring = springs
        .chars()
        .skip_while(|c| *c == '.')
        .collect::<String>();

    let mut count = 0;

    if normalized_spring.starts_with('?') {
        let mut new_string = normalized_spring.clone();
        new_string.replace_range(0..1, ".");

        count += search(spring_width_pointer, spring_widths, new_string, mem);
    }

    let mut current_width = spring_widths[spring_width_pointer] as i32;
    for (index, spring) in normalized_spring.clone().chars().enumerate() {
        match spring {
            '?' => {
                current_width -= 1;
            }
            '#' => {
                current_width -= 1;
            }
            '.' => {
                break;
            }
            _ => panic!("Unable to parse spring {spring}"),
        }

        if current_width == 0 {
            let mut remaining_springs = normalized_spring.clone()[index + 1..].to_string();
            if !remaining_springs.starts_with("#") {
                if remaining_springs.len() != 0 {
                    remaining_springs.replace_range(0..1, ".");
                }
                count += search(
                    spring_width_pointer + 1,
                    spring_widths,
                    remaining_springs.clone(),
                    mem
                );
            }
            break;
        }
    }

    mem.insert((spring_width_pointer, springs), count);
    count
}

fn part1() -> Option<u64> {
    let records = read_input();
    let mut sum = 0;
    let mut mem = HashMap::default();
    for (springs, records) in records.iter() {
        let count = search(0, records, springs.clone(), &mut mem);
        sum += count
    }
    Some(sum)
}

fn part2() -> Option<u64> {
    let records = read_input();
    let sum = records
        .iter()
        .map(|(springs, records)| {
            let mut mem = HashMap::default();
            let expanded_springs = format!("{springs}?{springs}?{springs}?{springs}?{springs}");
            let expanded_records = records
                .iter()
                .cycle()
                .take(records.len() * 5)
                .map(|record| *record)
                .collect::<Vec<u64>>();
            search(0, &expanded_records, expanded_springs, &mut mem)
        })
        .sum();
    Some(sum)
}

fn main() {
    println!("--- Day 12: Hot Springs ---");
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
