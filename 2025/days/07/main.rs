mod vector2;
use std::{collections::HashSet, fmt::Display};

use vector2::{Vector2, DOWN, LEFT, RIGHT};

#[derive(Debug, Clone)]
enum TileType {
    Empty,
    Start,
    Split,
    Beam(usize),
}

impl TileType {
    fn from_char(c: &char) -> Self {
        match c {
            '.' => Self::Empty,
            'S' => Self::Start,
            '^' => Self::Split,
            _ => panic!("Unknown tile {c}"),
        }
    }

    fn get_split_count(&self) -> usize {
        match self {
            Self::Start => 1,
            Self::Beam(base) => *base,
            _ => panic!("Oh no"),
        }
    }
}

#[derive(Debug, Clone)]
struct Tile {
    tile_type: TileType,
    position: Vector2,
}

impl Tile {
    fn new(position: Vector2, tile_char: &char) -> Self {
        Self {
            tile_type: TileType::from_char(tile_char),
            position,
        }
    }

    fn bump(&mut self, default: &TileType) {
        let default_base = default.get_split_count();

        match self.tile_type {
            TileType::Empty => self.tile_type = TileType::Beam(default_base),
            TileType::Beam(base) => self.tile_type = TileType::Beam(base + default_base),
            _ => {}
        }
    }

    fn pass(&mut self, default: &TileType) {
        let default_base = default.get_split_count();
        match self.tile_type {
            TileType::Beam(base) => self.tile_type = TileType::Beam(base + default_base),
            _ => self.tile_type = TileType::Beam(default_base),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.tile_type {
            TileType::Beam(base) => write!(f, "{}", base),
            TileType::Empty => write!(f, "."),
            TileType::Split => write!(f, "^"),
            TileType::Start => write!(f, "S"),
        }
    }
}

type Grid = Vec<Vec<Tile>>;

fn print_grid(grid: &Grid) {
    print!("\n");
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            print!("{}", grid[y][x]);
        }
        print!("\n");
    }
}

fn read_input() -> (Grid, Vector2) {
    let mut grid = Vec::default();
    let mut start: Option<Vector2> = None;

    for (y, line) in std::io::stdin()
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| line.trim().to_string())
        .enumerate()
    {
        let mut row = Vec::default();

        for (x, tile_char) in line.chars().enumerate() {
            let tile = Tile::new(Vector2::new(x, y), &tile_char);
            if matches!(tile.tile_type, TileType::Start) {
                start = Some(tile.position.clone())
            }
            row.push(tile);
        }

        grid.push(row);
    }
    (grid, start.unwrap())
}

fn get_tile_at(position: Vector2, grid: &Grid) -> Option<Vector2> {
    if let Some(row) = grid.get(position.1 as usize) {
        if let Some(_) = row.get(position.0 as usize) {
            return Some(position);
        }
    }
    None
}

type StepResult = (Option<Vector2>, Option<Vector2>, Option<Vector2>);

fn step(end: Vector2, grid: &Grid, split: &mut usize) -> StepResult {
    let mut down = None;
    let mut split_left = None;
    let mut split_right = None;

    if let Some(next_position) = get_tile_at(end.add(&DOWN), grid) {
        let tile_type = grid[next_position.1 as usize][next_position.0 as usize]
            .tile_type
            .clone();
        match tile_type {
            TileType::Empty => {
                // Becomes a new end
                down = Some(next_position);
                // grid[next_position.1 as usize][next_position.0 as usize].tile_type = TileType::Beam;
            }
            TileType::Split => {
                *split += 1;
                if let Some(left) = get_tile_at(next_position.add(&LEFT), grid) {
                    // grid[left.1 as usize][left.0 as usize].tile_type = TileType::Beam;
                    split_left = Some(left);
                }
                if let Some(right) = get_tile_at(next_position.add(&RIGHT), grid) {
                    // grid[right.1 as usize][right.0 as usize].tile_type = TileType::Beam;
                    split_right = Some(right);
                }
            }
            TileType::Beam(_) => {
                down = Some(next_position);
            }
            _ => {
                panic!("Invalid tile type {:?} => {next_position}", tile_type)
            }
        }
    }
    (down, split_left, split_right)
}

fn part1() -> Option<usize> {
    let (grid, start) = read_input();
    let mut ends: HashSet<Vector2> = HashSet::default();
    ends.insert(start);

    let mut split = 0;

    while ends.len() > 0 {
        let mut new_ends = HashSet::default();

        for end in ends {
            let (down, left, right) = step(end, &grid, &mut split);
            if let Some(down_position) = down {
                new_ends.insert(down_position);
            }
            if let Some(left_position) = left {
                new_ends.insert(left_position);
            }
            if let Some(right_position) = right {
                new_ends.insert(right_position);
            }
        }
        ends = new_ends
    }
    Some(split)
}

fn part2() -> Option<usize> {
    let (mut grid, start) = read_input();
    let mut ends: HashSet<Vector2> = HashSet::default();
    ends.insert(start);

    let mut timeline_count = 0;

    while ends.len() > 0 {
        let mut new_ends = HashSet::default();

        for end in ends {
            let tile = &grid[end.1 as usize][end.0 as usize].tile_type.clone();
            let (down, left, right) = step(end, &grid, &mut 0);
            if let Some(down_position) = down {
                grid[down_position.1 as usize][down_position.0 as usize].pass(tile);
                new_ends.insert(down_position);
            } else if left.is_some() || right.is_some() {
                if let Some(left_position) = left {
                    grid[left_position.1 as usize][left_position.0 as usize].bump(tile);
                    new_ends.insert(left_position);
                }
                if let Some(right_position) = right {
                    grid[right_position.1 as usize][right_position.0 as usize].bump(tile);
                    new_ends.insert(right_position);
                }
            } else {
                timeline_count += tile.get_split_count();
            }
        }
        ends = new_ends
    }
    Some(timeline_count)
}

fn main() {
    println!("--- Day 7: Laboratories ---");
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
