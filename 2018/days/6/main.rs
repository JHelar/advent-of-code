use std::{collections::{HashMap, HashSet}, fmt::Display};

type Position = (i32, i32);

#[derive(Debug)]
enum Tile {
    Empty,
    Single { dist: i32, pos: Position },
    Multiple { dist: i32, pos: Vec<Position> },
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Self::Empty => " ".to_string(),
            Self::Multiple { dist: _, pos: _ } => ".".to_string(),
            Self::Single { dist: _, pos } => {
                let char_digit = 97 + ((pos.0 + pos.1) % (122 - 97)) as u32;
                char::from_u32(char_digit).unwrap().to_string()
            }
        };

        write!(f, "{}", c)
    }
}

fn read_input() -> Vec<Position> {
    std::io::stdin()
        .lines()
        .filter(|line| line.is_ok())
        .map(|line| line.unwrap().trim().to_string())
        .map(|line| {
            let (x, y) = line.split_once(", ").unwrap();
            (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap())
        })
        .collect()
}

fn distance(a: Position, b: Position) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn print_map(map: &Vec<Vec<Tile>>, positions: &Vec<Position>) {
    for (y, row) in map.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            let mut c = tile.to_string();
            if positions.contains(&(x as i32, y as i32)) {
                c = c.to_uppercase();
            }
            print!("{c}");
        }
        print!("\n");
    }
}

fn part1() -> Option<u32> {
    let positions = read_input();
    let max_x = *positions.iter().map(|(x, _)| x).max().unwrap();
    let max_y = *positions.iter().map(|(_, y)| y).max().unwrap();
    
    let mut map: Vec<Vec<Tile>> = (0..=max_y)
        .map(|_| (0..=max_x).map(|_| Tile::Empty).collect())
        .collect();

    for position in positions.iter() {
        for y in 0..=max_y {
            for x in 0..=max_x {
                let distance = distance((x as i32, y as i32), *position);
                let tile = &mut map[y as usize][x as usize];

                match tile {
                    Tile::Empty => {
                        map[y as usize][x as usize] = Tile::Single {
                            dist: distance,
                            pos: *position,
                        }
                    }
                    Tile::Single { dist, pos } if *dist == distance => {
                        map[y as usize][x as usize] = Tile::Multiple {
                            dist: distance,
                            pos: vec![*pos, *position],
                        }
                    }
                    Tile::Single { dist, pos: _ } if *dist > distance => {
                        map[y as usize][x as usize] = Tile::Single {
                            dist: distance,
                            pos: *position,
                        }
                    }
                    Tile::Multiple { dist, pos } if *dist == distance => {
                        pos.push(*position);
                    }
                    Tile::Multiple { dist, pos: _ } if *dist > distance => {
                        map[y as usize][x as usize] = Tile::Single {
                            dist: distance,
                            pos: *position,
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    let mut finite_positions = positions.iter().map(|pos| (pos, 0_u32)).collect::<HashMap<_,_>>();

    map.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, tile)| {
            if y == 0 || (y as i32) == max_y || x == 0 || (x as i32) == max_x {
                match tile {
                    Tile::Single { dist: _, pos } => {
                        finite_positions.remove(pos);
                    },
                    _ => {}
                }
            } else {
                match tile {
                    Tile::Single { dist: _, pos } => {
                        if let Some(count) = finite_positions.get_mut(pos) {
                            *count += 1;
                        }
                    },
                    _ => {}
                }
            }
        })
    });

    finite_positions.iter().map(|(_, count)| *count).max()
}

fn part2() -> Option<u32> {
    let positions = read_input();
    let max_x = *positions.iter().map(|(x, _)| x).max().unwrap();
    let max_y = *positions.iter().map(|(_, y)| y).max().unwrap();
    let max_distance = 10_000;
    let mut area_count = 0_u32;
    
    let mut map: Vec<Vec<Tile>> = (0..=max_y)
        .map(|_| (0..=max_x).map(|_| Tile::Empty).collect())
        .collect();
    
        for y in 0..=max_y {
            for x in 0..=max_x {
                let distance = positions.iter().map(|position| distance((x as i32, y as i32), *position)).sum();
                let tile = &mut map[y as usize][x as usize];

                if distance < max_distance {
                    match tile {
                        Tile::Empty => {
                            area_count += 1;
                            map[y as usize][x as usize] = Tile::Single {
                                dist: distance,
                                pos: (0, 0),
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

    Some(area_count)
}

fn main() {
    println!("--- Day 6: Chronal Coordinates ---");
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
