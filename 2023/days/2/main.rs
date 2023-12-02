use std::collections::HashMap;

type Game = HashMap<String, u32>;
type GameSets = Vec<Game>;

fn read_input() -> Vec<GameSets> {
    std::io::stdin()
        .lines()
        .filter(|line| line.is_ok())
        .map(|line| line.unwrap().trim().to_string())
        .map(|line| {
            let (_, sets) = line.split_once(": ").unwrap();
            let mut game_sets: GameSets = Vec::default();
            sets.split("; ").for_each(|cubes| {
                let mut game: Game = HashMap::default();
                cubes.split(", ").for_each(|cube_str| {
                    let (amount_str, color) = cube_str.split_once(" ").unwrap();

                    if let Some(amount) = game.get_mut(&color.to_string()) {
                        *amount += amount_str.parse::<u32>().unwrap()
                    } else {
                        game.insert(color.to_string(), amount_str.parse().unwrap());
                    }
                });
                game_sets.push(game);
            });
            game_sets
        })
        .collect()
}

fn part1() -> Option<u32> {
    let games = read_input();

    let result = games
        .iter()
        .enumerate()
        .filter(|(_, game_sets)| {
            if game_sets.iter().any(|game| {
                if let Some(count) = game.get(&"red".to_string()) {
                    *count > 12
                } else {
                    false
                }
            }) {
                return false;
            }
            
            if game_sets.iter().any(|game| {
                if let Some(count) = game.get(&"green".to_string()) {
                    *count > 13
                } else {
                    false
                }
            }) {
                return false;
            }

            if game_sets.iter().any(|game| {
                if let Some(count) = game.get(&"blue".to_string()) {
                    *count > 14
                } else {
                    false
                }
            }) {
                return false;
            }

            true
        })
        .map(|(i, _)| (i + 1) as u32)
        .sum();

    Some(result)
}

fn part2() -> Option<u32> {
    let games = read_input();

    let result = games
        .iter()
        .map(|game_set| {
            let red_count = game_set
                .iter()
                .map(|game| game.get(&"red".to_string()).unwrap_or(&0_u32))
                .max()
                .unwrap();
            let green_count = game_set
                .iter()
                .map(|game| game.get(&"green".to_string()).unwrap_or(&0_u32))
                .max()
                .unwrap();
            let blue_count = game_set
                .iter()
                .map(|game| game.get(&"blue".to_string()).unwrap_or(&0_u32))
                .max()
                .unwrap();

            red_count * green_count * blue_count
        })
        .sum();

    Some(result)
}

fn main() {
    println!("--- Day 2: Cube Conundrum ---");
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
