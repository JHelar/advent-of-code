mod vector2;
use std::fmt::Display;

use vector2::{Vector2, DOWN, LEFT, RIGHT, UP};

type Grid = Vec<Vec<Tile>>;

const DIRECTIONS: [Vector2; 8] = [
    UP,
    DOWN,
    RIGHT,
    LEFT,
    Vector2(-1, -1),
    Vector2(1, -1),
    Vector2(-1, 1),
    Vector2(1, 1),
];

#[derive(Debug)]
enum TileType {
    Paper,
    Empty,
    Forklift,
}

impl Display for TileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "."),
            Self::Paper => write!(f, "@"),
            Self::Forklift => write!(f, "x"),
        }
    }
}

impl TileType {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '@' => Self::Paper,
            _ => panic!("Invalid tile char {c}"),
        }
    }
}

#[derive(Debug)]
struct Tile {
    position: Vector2,
    tile_type: TileType,
}

impl Tile {
    fn from_char(c: char, position: Vector2) -> Self {
        Self {
            position,
            tile_type: TileType::from_char(c),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.tile_type)
    }
}

fn print_grid(grid: &Grid) {
    print!("\n");
    for row in grid.iter() {
        for tile in row.iter() {
            print!("{tile}")
        }
        print!("\n")
    }
}

fn read_input() -> Grid {
    std::io::stdin()
        .lines()
        .filter_map(|line| line.ok())
        .enumerate()
        .map(|(y, line)| {
            line.trim()
                .chars()
                .enumerate()
                .map(|(x, c)| Tile::from_char(c, Vector2::new(x, y)))
                .collect::<Vec<Tile>>()
        })
        .collect::<Grid>()
}

fn neighbors(position: Vector2, grid: &Grid) -> Vec<&Tile> {
    DIRECTIONS
        .iter()
        .filter_map(|direction| {
            let neighbor_position = position.add(direction);
            if neighbor_position.1 < 0 || neighbor_position.1 >= grid.len() as isize {
                None
            } else if let Some(row) = grid.get(neighbor_position.1 as usize) {
                if neighbor_position.0 < 0 || neighbor_position.0 >= row.len() as isize {
                    None
                } else {
                    row.get(neighbor_position.0 as usize)
                }
            } else {
                None
            }
        })
        .collect()
}

fn part1() -> Option<usize> {
    let mut grid = read_input();
    print_grid(&grid);
    let mut hits = 0;
    for y in 0..grid.len() {
        let width = grid[y].len();
        for x in 0..width {
            match grid[x][y] {
                Tile {
                    position,
                    tile_type: TileType::Paper,
                } => {
                    let papers = neighbors(position, &grid)
                        .iter()
                        .filter(|tile| {
                            matches!(tile.tile_type, TileType::Paper)
                                || matches!(tile.tile_type, TileType::Forklift)
                        })
                        .count();

                    if papers < 4 {
                        grid[x][y].tile_type = TileType::Forklift;
                        hits += 1
                    }
                }
                _ => {}
            }
        }
    }
    print_grid(&grid);
    Some(hits)
}

fn part2() -> Option<usize> {
    let mut grid = read_input();
    print_grid(&grid);
    let mut hits = 0;

    loop {
        let mut did_remove = false;
        for y in 0..grid.len() {
            let width = grid[y].len();
            for x in 0..width {
                match grid[x][y] {
                    Tile {
                        position,
                        tile_type: TileType::Paper,
                    } => {
                        let papers = neighbors(position, &grid)
                            .iter()
                            .filter(|tile| {
                                matches!(tile.tile_type, TileType::Paper)
                                    || matches!(tile.tile_type, TileType::Forklift)
                            })
                            .count();

                        if papers < 4 {
                            did_remove = true;
                            grid[x][y].tile_type = TileType::Forklift;
                            hits += 1
                        }
                    }
                    _ => {}
                }
            }
        }
        for y in 0..grid.len() {
            let width = grid[y].len();
            for x in 0..width {
                match grid[x][y] {
                    Tile {
                        position: _,
                        tile_type: TileType::Forklift,
                    } => grid[x][y].tile_type = TileType::Empty,
                    _ => {}
                }
            }
        }
        if !did_remove {
            break;
        }
        print_grid(&grid);
    }
    Some(hits)
}

fn main() {
    println!("--- Day 4: Printing Department ---");
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
