use std::collections::HashMap;

type Stones = Vec<usize>;

fn read_input() -> Stones {
    let mut stone_line = String::new();

    let _ = std::io::stdin().read_line(&mut stone_line);

    stone_line
        .trim()
        .split_whitespace()
        .map(|num| num.parse::<usize>().unwrap())
        .collect()
}

fn step(state: &HashMap<usize, isize>) -> HashMap<usize, isize> {
    let mut new_state = HashMap::new();

    for stone in state.keys() {
        match stone {
            stone if *stone == 0 => {
                *new_state.entry(1).or_insert(0) += state.get(stone).or_else(|| Some(&1)).unwrap();
            }
            stone if format!("{stone}").len() % 2 == 0 => {
                let stone_str = format!("{stone}");
                let (left_str, right_str) = stone_str.split_at(stone_str.len() / 2);

                *new_state.entry(left_str.parse().unwrap()).or_insert(0) +=
                    state.get(stone).or_else(|| Some(&1)).unwrap();
                *new_state.entry(right_str.parse().unwrap()).or_insert(0) +=
                    state.get(stone).or_else(|| Some(&1)).unwrap();
            }
            stone => {
                *new_state.entry(stone * 2024).or_insert(0) +=
                    state.get(stone).or_else(|| Some(&1)).unwrap();
            }
        }
    }

    new_state
}

fn part1() -> Option<isize> {
    let stones = read_input();
    let state = stones.into_iter().map(|stone| (stone, 1)).collect();
    let result = (0..25).fold(state, |state, _| step(&state)).values().sum();
    Some(result)
}

fn part2() -> Option<isize> {
    let stones = read_input();
    let state = stones.into_iter().map(|stone| (stone, 1)).collect();
    let result = (0..75).fold(state, |state, _| step(&state)).values().sum();

    Some(result)
}

fn main() {
    println!("--- Day 11: Plutonian Pebbles ---");
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
