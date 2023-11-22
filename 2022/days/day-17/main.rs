use std::collections::HashMap;
use std::env;
use std::fmt::Display;
use std::fs;

type Pos = (i32, i32);

const BOARD_WIDTH: usize = 7;
const ROCK_COUNT: usize = 5;
const CAVE_CEILING_LIMIT: i32 = 20;
static ROCK_ORDER: [Rock; ROCK_COUNT] = [
    Rock::Horizontal,
    Rock::Plus,
    Rock::L,
    Rock::Vertical,
    Rock::Box,
];

#[derive(Debug, Copy, Clone)]
enum Rock {
    Horizontal,
    Plus,
    L,
    Vertical,
    Box,
}

impl Rock {
    fn get_height(&self) -> i32 {
        match self {
            Rock::Horizontal => 1,
            Rock::Plus => 3,
            Rock::L => 3,
            Rock::Vertical => 4,
            Rock::Box => 2,
        }
    }

    fn get_shape_deltas(&self) -> Vec<Pos> {
        match self {
            Rock::Horizontal => {
                vec![(0, 0), (1, 0), (2, 0), (3, 0)]
            }
            Rock::Plus => {
                vec![(1, 0), (0, -1), (1, -1), (2, -1), (1, -2)]
            }
            Rock::L => {
                vec![(2, 0), (2, -1), (0, -2), (1, -2), (2, -2)]
            }
            Rock::Vertical => {
                vec![(0, 0), (0, -1), (0, -2), (0, -3)]
            }
            Rock::Box => {
                vec![(0, 0), (1, 0), (0, -1), (1, -1)]
            }
        }
    }
}

#[derive(Debug)]
enum Gas {
    Left,
    Right,
    Down,
}

impl Gas {
    fn from_char(gas_char: char) -> Self {
        match gas_char {
            '>' => Gas::Right,
            '<' => Gas::Left,
            _ => todo!("No such gas direction"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum CaveTile {
    Air,
    Rock,
    Moving,
}

impl Display for CaveTile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CaveTile::Air => write!(f, "."),
            CaveTile::Rock => write!(f, "#"),
            CaveTile::Moving => write!(f, "@"),
        }
    }
}

#[derive(Debug)]
struct Board {
    initialied: bool,
    rock_pointer: usize,
    rock_pos: Pos,
    cave_ceiling: i32,
    cave: Vec<[CaveTile; BOARD_WIDTH]>,
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let current_rock = ROCK_ORDER[self.rock_pointer];
        let binding = current_rock.get_shape_deltas();

        let rock_shape: Vec<Pos> = binding
            .iter()
            .map(|(x_delta, y_delta)| (self.rock_pos.0 + x_delta, self.rock_pos.1 + y_delta))
            .collect();

        for (y, row) in self.cave.iter().enumerate().rev() {
            write!(f, "|").unwrap();
            for (x, cave_tile) in row.iter().enumerate() {
                let tile = if rock_shape
                    .iter()
                    .any(|(rock_x, rock_y)| *rock_x == (x as i32) && *rock_y == (y as i32))
                {
                    &CaveTile::Moving
                } else {
                    cave_tile
                };
                write!(f, "{}", tile).unwrap();
            }
            write!(f, "|\n").unwrap();
        }

        writeln!(f, "+{:-<width$}+", "-", width = BOARD_WIDTH)
    }
}

impl Board {
    fn new() -> Self {
        Self {
            initialied: false,
            rock_pointer: 0,
            rock_pos: (0, 0),
            cave_ceiling: -1,
            cave: vec![],
        }
    }

    fn new_rock(&mut self) {
        self.rock_pointer = if self.initialied {
            (self.rock_pointer + 1) % ROCK_COUNT
        } else {
            self.initialied = true;
            0
        };

        let rock = ROCK_ORDER[self.rock_pointer];
        self.rock_pos = (2, self.cave_ceiling + rock.get_height() + 3);
        let additional_height: i32 = (self.rock_pos.1 + 1) - (self.cave.len() as i32);

        for _i in 0..additional_height {
            self.cave.push([
                CaveTile::Air,
                CaveTile::Air,
                CaveTile::Air,
                CaveTile::Air,
                CaveTile::Air,
                CaveTile::Air,
                CaveTile::Air,
            ]);
        }
    }

    fn move_rock(&mut self, gas: &Gas) -> bool {
        let current_rock = ROCK_ORDER[self.rock_pointer];
        let new_pos = match gas {
            Gas::Left => {
                if self.rock_pos.0 > 0 {
                    (self.rock_pos.0 - 1, self.rock_pos.1)
                } else {
                    return false;
                }
            }
            Gas::Right => {
                if self.rock_pos.0 < (BOARD_WIDTH as i32) - 1 {
                    (self.rock_pos.0 + 1, self.rock_pos.1)
                } else {
                    return false;
                }
            }
            Gas::Down => {
                if self.rock_pos.1 > 0 {
                    (self.rock_pos.0, self.rock_pos.1 - 1)
                } else {
                    return false;
                }
            }
        };
        if self.can_move_rock(new_pos, current_rock.get_shape_deltas()) {
            self.rock_pos = new_pos;
            return true;
        }
        false
    }

    fn can_move_rock(&self, new_pos: Pos, rock_edges_deltas: Vec<Pos>) -> bool {
        for (delta_x, delta_y) in rock_edges_deltas {
            let x = new_pos.0 + delta_x;
            let y = new_pos.1 + delta_y;

            let row = if let Some(row) = self.cave.get(y as usize) {
                row
            } else {
                return false;
            };
            let tile = if let Some(tile) = row.get(x as usize) {
                tile
            } else {
                return false;
            };

            if matches!(tile, CaveTile::Rock) {
                return false;
            }
        }

        true
    }

    fn settle_rock(&mut self) {
        let current_rock = ROCK_ORDER[self.rock_pointer];
        let rock_shape = current_rock.get_shape_deltas();

        for (x_delta, y_delta) in rock_shape {
            let x = self.rock_pos.0 + x_delta;
            let y = self.rock_pos.1 + y_delta;

            if self.cave_ceiling < y {
                self.cave_ceiling = y;
            }

            self.cave[y as usize][x as usize] = CaveTile::Rock
        }
    }

    fn cave_height(&self) -> i32 {
        self.cave_ceiling + 1
    }

    fn cave_state(&self) -> Option<String> {
        if self.cave_height() < CAVE_CEILING_LIMIT {
            return None;
        }

        let air_pocket = self.cave.len() - self.cave_height() as usize;
        let top_rows = self.cave.iter().rev().skip(air_pocket).take(CAVE_CEILING_LIMIT as usize);

        let state = top_rows.fold("".to_string(), |acc, row| {
            format!("{acc}{}", row.map(|tile| tile.to_string()).join(""))
        });

        Some(state)
    }
}

fn parse_input() -> Vec<Gas> {
    fs::read_to_string("input.txt")
        .expect("Unable to read file!")
        .trim_end()
        .chars()
        .map(|gas_str| Gas::from_char(gas_str))
        .collect()
}

fn drop_rock(board: &mut Board, initial_gas_pointer: usize, gas: &Vec<Gas>) -> usize {
    let mut gas_pointer = initial_gas_pointer;
    loop {
        board.move_rock(&gas[gas_pointer]);
        gas_pointer = (gas_pointer + 1) % gas.len();
        
        if board.move_rock(&Gas::Down) {
            continue;
        }
        board.settle_rock();
        break;
    }
    gas_pointer
}

fn part1() {
    let gas = parse_input();
    let mut board = Board::new();

    let mut gas_pointer: usize = 0;
    let rocks = 2022;

    for _rock_count in 0..rocks {
        board.new_rock();
        gas_pointer = drop_rock(&mut board, gas_pointer, &gas);
    }
    println!("Cave height: {}", board.cave_height());
}

fn part2() {
    let gas = parse_input();
    let mut board = Board::new();

    let mut gas_pointer: usize = 0;
    let total_rocks: i64 = 1_000_000_000_000;
    let mut rocks = 0;
    let mut accumulated_height = 0;

    let mut board_states = HashMap::with_capacity(2048);

    while rocks < total_rocks {
        board.new_rock();
        gas_pointer = drop_rock(&mut board, gas_pointer, &gas);

        rocks += 1;

        if board.cave_height() < CAVE_CEILING_LIMIT {
            continue;
        }

        let state = (board.cave_state(), board.rock_pointer, gas_pointer);
        if let Some((prev_rocks, prev_cave_height)) = board_states.get(&state) {
            let repeat_len = rocks - prev_rocks;
            let repeats = (total_rocks - rocks) / repeat_len;

            rocks += repeat_len * repeats;
            accumulated_height += repeats * (board.cave_height() - prev_cave_height) as i64;

            board_states.clear();
        }
        board_states.insert(state, (rocks, board.cave_height()));
    }
    let total_height = board.cave_height() as i64 + accumulated_height;
    println!("{board}");
    println!("Cave height: {total_height}");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let part = &args[1];

    if part == "1" {
        part1();
    } else if part == "2" {
        part2();
    }
}
