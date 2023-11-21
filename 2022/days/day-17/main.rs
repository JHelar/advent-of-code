use std::env;
use std::fs;

#[derive(Debug)]
enum Rock {
    Horizontal,
    Plus,
    L,
    Vertical,
    Box,
}

#[derive(Debug)]
enum Gas {
    Left,
    Right,
}

impl Gas {
    fn from_char(gas_char: char) -> Self {
        print!("{}",gas_char);
        match gas_char {
            '>' => Gas::Right,
            '<' => Gas::Left,
            _ => todo!("No such gas direction"),
        }
    }
}

#[derive(Debug)]
enum BoardState {
    Air,
    Rock
}

#[derive(Debug)]
struct Board {
    rock_pointer: usize,
    rock_pos: (usize, usize),
    cave_ceiling: usize,
    cave: Vec<[BoardState; 7]>
}

impl Board {
    fn new() -> Self {
        Self {
            rock_pointer: 0,
            rock_pos: (0, 0),
            cave_ceiling: 0,
            cave: vec![]
        }
    }
}

static ROCK_ORDER: [Rock; 5] = [Rock::Horizontal, Rock::Plus, Rock::L, Rock::Vertical, Rock::Box];

fn parse_input() -> Vec<Gas> {
    fs::read_to_string("input.txt")
        .expect("Unable to read file!")
        .trim_end()
        .chars()
        .map(|gas_str| Gas::from_char(gas_str))
        .collect()
}

fn part1() {
    let gas = parse_input();
    let board = Board::new();
    println!("{:?}", gas);
}

fn part2() {}

fn main() {
    let args: Vec<String> = env::args().collect();
    let part = &args[1];

    if part == "1" {
        part1();
    } else if part == "2" {
        part2();
    }
}
