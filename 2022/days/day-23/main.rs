use std::collections::{HashMap, VecDeque};
use std::env;
use std::fmt::Display;
use std::fs;

type Map = Vec<Vec<Tile>>;
type Offset = (i32, i32);
type Position = (i32, i32);
type OffsetRule = ([Offset; 3], Offset);

const W: Offset = (-1, 0);
const E: Offset = (1, 0);
const N: Offset = (0, -1);
const S: Offset = (0, 1);
const SW: Offset = (-1, 1);
const SE: Offset = (1, 1);
const NW: Offset = (-1, -1);
const NE: Offset = (1, -1);

const ALL_OFFSETS: [Offset; 8] = [W, E, N, S, SW, SE, NW, NE];
const OFFSETS_N: OffsetRule = ([N, NE, NW], N);
const OFFSETS_S: OffsetRule = ([S, SE, SW], S);
const OFFSETS_W: OffsetRule = ([W, NW, SW], W);
const OFFSETS_E: OffsetRule = ([E, NE, SE], E);

const MAP_PADDING: usize = 1000;

#[derive(Debug, Clone, Copy)]
enum Tile {
    Elf,
    Ground,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Tile::Elf => '#',
            Tile::Ground => '.',
        };
        write!(f, "{c}")
    }
}

fn parse_input() -> Map {
    let mut map = fs::read_to_string("input.txt")
        .expect("Unable to read file!")
        .lines()
        .map(|line| line.trim())
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Tile::Ground,
                    '#' => Tile::Elf,
                    _ => todo!("No tile matching {c}"),
                })
                .collect::<Vec<Tile>>()
        })
        .collect::<Map>();

    for y in 0..map.len() {
        for _ in 0..MAP_PADDING {
            map[y].push(Tile::Ground);
            map[y].insert(0, Tile::Ground);
        }
    }

    for _ in 0..MAP_PADDING {
        map.insert(0, (0..map[0].len()).map(|_| Tile::Ground).collect());
        map.push((0..map[0].len()).map(|_| Tile::Ground).collect());
    }
    map
}

fn has_adjacent_elfs(elf: Position, map: &Map) -> bool {
    ALL_OFFSETS
        .iter()
        .map(|offset| add_offset(elf, *offset))
        .any(|(x, y)| {
            if y > -1 && map.get(y as usize).is_some() {
                let row = map.get(y as usize).unwrap();
                if x > -1 && row.get(x as usize).is_some() {
                    return matches!(row.get(x as usize).unwrap(), Tile::Elf);
                }
            }
            false
        })
}

fn is_empty(elf: Position, offsets: [Offset; 3], map: &Map) -> bool {
    offsets
        .iter()
        .map(|offset| add_offset(elf, *offset))
        .all(|(x, y)| {
            if y > -1 && map.get(y as usize).is_some() {
                let row = map.get(y as usize).unwrap();
                if x > -1 && row.get(x as usize).is_some() {
                    return matches!(row.get(x as usize).unwrap(), Tile::Ground);
                }
            }
            panic!("Hit the end of the world: ({x}, {y})");
        })
}

fn print_map(map: &Map) {
    print!("\n");
    for row in map {
        for tile in row {
            print!("{tile}");
        }
        print!("\n");
    }
}

fn add_offset((elf_x, elf_y): Position, (delta_x, delta_y): Offset) -> Position {
    (elf_x + delta_x, elf_y + delta_y)
}

fn simulate(map: &mut Map, rules: &VecDeque<OffsetRule>) -> bool {
    let mut suggestions: HashMap<Position, Vec<Position>> = HashMap::default();

    for (y, row) in map.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if matches!(tile, Tile::Elf) {
                let elf = (x as i32, y as i32);

                if has_adjacent_elfs(elf, map) {
                    for (offsets, offset) in rules.iter() {
                        if is_empty(elf, *offsets, map) {
                            let pos = add_offset(elf, *offset);
                            if let Some(existing) = suggestions.get_mut(&pos) {
                                existing.push(elf);
                            } else {
                                suggestions.insert(pos, vec![elf]);
                            }
                            break;
                        }
                    }
                }
            }
        }
    }

    if suggestions.is_empty() {
        return false;
    }

    for ((to_x, to_y), elfs) in suggestions {
        if elfs.len() > 1 {
            continue;
        }
        let (elf_x, elf_y) = elfs[0];

        map[elf_y as usize][elf_x as usize] = Tile::Ground;
        map[to_y as usize][to_x as usize] = Tile::Elf;
    }

    true
}

fn part1() {
    let mut map = parse_input();
    let mut rules: VecDeque<_> = [OFFSETS_N, OFFSETS_S, OFFSETS_W, OFFSETS_E].into();

    for _ in 0..10 {
        simulate(&mut map, &rules);
        rules.rotate_left(1);
    }

    let x_coords: Vec<usize> = map
        .iter()
        .flat_map(|row| {
            row.iter()
                .enumerate()
                .filter(|(_, tile)| matches!(tile, Tile::Elf))
                .map(|(x, _)| x)
        })
        .collect();

    let y_coords: Vec<usize> = map
        .iter()
        .enumerate()
        .filter(|(_, row)| row.iter().any(|tile| matches!(tile, Tile::Elf)))
        .map(|(y, _)| y)
        .collect();

    let x_max = *x_coords.iter().max().unwrap();
    let x_min = *x_coords.iter().min().unwrap();

    let y_max = *y_coords.iter().max().unwrap();
    let y_min = *y_coords.iter().min().unwrap();

    let mut sum: usize = 0;
    for row in &mut map[y_min..=y_max] {
        let cut_row = row[x_min..=x_max].to_vec();
        sum += cut_row.iter().filter(|tile| matches!(tile, Tile::Ground)).count();
    }

    println!("Result: {sum}");
}

fn part2() {
    let mut map = parse_input();
    let mut rules: VecDeque<_> = [OFFSETS_N, OFFSETS_S, OFFSETS_W, OFFSETS_E].into();

    let mut iteration = 1;
    while simulate(&mut map, &rules) {
        rules.rotate_left(1);
        iteration += 1;   
    }

    println!("Result: {iteration}");
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
