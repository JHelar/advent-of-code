use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
};

#[derive(Debug)]
struct MarbleGame {
    circle: VecDeque<u32>,
    current_marble_value: u32,
}

impl MarbleGame {
    fn new() -> Self {
        Self {
            circle: VecDeque::from([0]),
            current_marble_value: 0
        }
    }

    fn next(&mut self) -> Option<u32> {
        self.current_marble_value += 1;

        if (self.current_marble_value % 23) == 0 {
            self.circle.rotate_right(7);
            let result = self.current_marble_value + self.circle.pop_back().unwrap();
            self.circle.rotate_left(1);

            Some(result)
        } else {
            self.circle.rotate_left(1);
            self.circle.push_back(self.current_marble_value);

            None
        }
    }
}

fn read_input() -> (usize, u32) {
    let mut line = String::new();
    let _ = std::io::stdin().read_line(&mut line);

    let (player_count_str, max_points_str) = line.trim().split_once("; ").unwrap();
    let player_count = player_count_str
        .replace(" players", "")
        .parse::<usize>()
        .unwrap();
      
    let max_points = max_points_str
        .replace("last marble is worth ", "")
        .replace(" points", "")
        .parse::<u32>()
        .unwrap();

    (player_count, max_points)
}

fn part1() -> Option<u32> {
    let mut game = MarbleGame::new();
    let (player_count, max_marble_value) = read_input();

    let mut players: Vec<u32> = vec![0; player_count];
    let mut current_player_index = 0;

    while game.current_marble_value <= max_marble_value {
        let current_player = players.get_mut(current_player_index).unwrap();
        if let Some(count) = game.next() {
            *current_player += count
        }

        current_player_index = (current_player_index + 1) % player_count;
    }

    players.iter().max().copied()
}

fn part2() -> Option<u32> {
    let mut game = MarbleGame::new();
    let (player_count, max_marble_value) = read_input();

    let mut players: Vec<u32> = vec![0; player_count];
    let mut current_player_index = 0;

    while game.current_marble_value <= (max_marble_value * 100) {
        let current_player = players.get_mut(current_player_index).unwrap();
        if let Some(count) = game.next() {
            *current_player += count
        }

        current_player_index = (current_player_index + 1) % player_count;
    }

    players.iter().max().copied()
}

fn main() {
    println!("--- Day 9: Marble Mania ---");
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
