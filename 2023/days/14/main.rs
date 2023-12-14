use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

#[derive(Debug, Clone, Copy)]
enum Tile {
    RoundRock,
    CubeRock,
    Empty,
}

impl Tile {
    fn from_char(c: char) -> Self {
        match c {
            'O' => Self::RoundRock,
            '#' => Self::CubeRock,
            '.' => Self::Empty,
            c => panic!("Unknown tile {c}"),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Self::CubeRock => '#',
            Self::RoundRock => 'O',
            Self::Empty => '.',
        };
        write!(f, "{c}")
    }
}

enum FallDirection {
    North,
    West,
    South,
    East,
}

fn read_input() -> Vec<Vec<Tile>> {
    std::io::stdin()
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| line.trim().chars().map(Tile::from_char).collect())
        .collect()
}

fn print_map(map: &Vec<Vec<Tile>>) {
    for row in map.iter() {
        for tile in row.iter() {
            print!("{tile}");
        }
        print!("\n");
    }
    print!("\n");
}

fn drop_rocks_north(map: &mut Vec<Vec<Tile>>) {
    let mut column_map = Vec::new();
    for x in 0..map[0].len() {
        let mut column = Vec::new();
        for y in 0..map.len() {
            column.push(map[y][x]);
        }
        column_map.push(column);
    }

    for (x, column) in column_map.iter_mut().enumerate() {
        let mut ground_index = 0;
        for y in 0..column.len() {
            let tile = column[y];
            match tile {
                Tile::RoundRock => {
                    *map.get_mut(y).unwrap().get_mut(x).unwrap() = Tile::Empty;
                    *map.get_mut(ground_index).unwrap().get_mut(x).unwrap() = Tile::RoundRock;
                    ground_index += 1
                }
                Tile::CubeRock => ground_index = y + 1,
                Tile::Empty => {}
            }
        }
    }
}

fn drop_rocks_south(map: &mut Vec<Vec<Tile>>) {
    let mut column_map = Vec::new();

    for x in 0..map[0].len() {
        let mut column = Vec::new();
        for y in (0..map.len()).rev() {
            column.push(map[y][x]);
        }
        column_map.push(column);
    }

    for (x, column) in column_map.iter_mut().enumerate() {
        let mut ground_index = 0;
        for y in 0..column.len() {
            let tile = column[y];
            match tile {
                Tile::RoundRock => {
                    *map.get_mut(column.len() - 1 - y)
                        .unwrap()
                        .get_mut(x)
                        .unwrap() = Tile::Empty;
                    *map.get_mut(column.len() - 1 - ground_index)
                        .unwrap()
                        .get_mut(x)
                        .unwrap() = Tile::RoundRock;
                    ground_index += 1
                }
                Tile::CubeRock => ground_index = y + 1,
                Tile::Empty => {}
            }
        }
    }
}

fn drop_rocks_west(map: &mut Vec<Vec<Tile>>) {
    let mut column_map = map.clone();

    for (x, column) in column_map.iter_mut().enumerate() {
        let mut ground_index = 0;
        for y in 0..column.len() {
            let tile = column[y];
            match tile {
                Tile::RoundRock => {
                    *map.get_mut(x).unwrap().get_mut(y).unwrap() = Tile::Empty;
                    *map.get_mut(x).unwrap().get_mut(ground_index).unwrap() = Tile::RoundRock;
                    ground_index += 1
                }
                Tile::CubeRock => ground_index = y + 1,
                Tile::Empty => {}
            }
        }
    }
}

fn drop_rocks_east(map: &mut Vec<Vec<Tile>>) {
    let mut column_map = map.clone();

    for (x, column) in column_map.iter_mut().enumerate() {
        let mut ground_index = column.len() - 1;
        for y in (0..column.len()).rev() {
            let tile = column[y];
            match tile {
                Tile::RoundRock => {
                    *map.get_mut(x).unwrap().get_mut(y).unwrap() = Tile::Empty;
                    *map.get_mut(x).unwrap().get_mut(ground_index).unwrap() = Tile::RoundRock;
                    if ground_index > 0 {
                        ground_index -= 1
                    }
                }
                Tile::CubeRock => {
                    if y > 0 {
                        ground_index = y - 1
                    }
                }
                Tile::Empty => {}
            }
        }
    }
}

fn drop_rocks(map: &mut Vec<Vec<Tile>>, fall_direction: FallDirection) {
    match fall_direction {
        FallDirection::North => drop_rocks_north(map),
        FallDirection::South => drop_rocks_south(map),
        FallDirection::East => drop_rocks_east(map),
        FallDirection::West => drop_rocks_west(map),
    }
}

fn cycle(map: &mut Vec<Vec<Tile>>) {
    drop_rocks(map, FallDirection::North);
    drop_rocks(map, FallDirection::West);
    drop_rocks(map, FallDirection::South);
    drop_rocks(map, FallDirection::East);
}

fn get_map_state(map: &Vec<Vec<Tile>>) -> String {
    map.iter()
        .map(|row| {
            row.iter()
                .map(|tile| tile.to_string())
                .reduce(|prev, current| format!("{prev}{current}"))
                .unwrap()
        })
        .reduce(|prev, current| format!("{prev}{current}"))
        .unwrap()
}

fn count_rocks(map: &Vec<Vec<Tile>>) -> u32 {
    let mut sum = 0;
    for (y, row) in map.iter().enumerate() {
        for tile in row.iter() {
            match tile {
                Tile::RoundRock => sum += (map.len() - y) as u32,
                _ => {}
            }
        }
    }
    sum
}

fn part1() -> Option<u32> {
    let mut map = read_input();
    drop_rocks(&mut map, FallDirection::North);
    let result = count_rocks(&map);

    print_map(&map);
    Some(result)
}

fn part2() -> Option<u32> {
    let mut map = read_input();
    let mut mem: HashMap<String, usize> = HashMap::default();
    mem.insert(get_map_state(&map), 0);

    let mut cycle_start = 0;
    let mut cycle_size = 0;
    for cycle_count in 1..18 {
        cycle(&mut map);
        let state = get_map_state(&map);
        if let Some(prev_cycle_count) = mem.get(&state) {
          cycle_size = cycle_count - prev_cycle_count;
          cycle_start = cycle_count;
          break;
        } else {
          mem.insert(state, cycle_count);
        }
    }

    for cycle_count in (cycle_start..1000000000).step_by(cycle_size) {
      cycle(&mut map);
    }

    print_map(&map);
    Some(0)
}

fn main() {
    println!("--- Day 14: Parabolic Reflector Dish ---");
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
