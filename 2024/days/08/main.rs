use crossterm::{
    cursor,
    style::{self, style, Stylize},
    terminal, ExecutableCommand, QueueableCommand,
};
use std::{
    collections::HashMap,
    fmt::Display,
    io::{self, Write},
    thread::sleep,
    time::Duration,
};
use vector2::Vector2;

mod vector2;

#[derive(Debug)]
enum Tile {
    Space,
    Antenna(char, usize),
    Antinode,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Space => write!(f, "."),
            Tile::Antinode => write!(f, "#"),
            Tile::Antenna(name, _) => write!(f, "{name}"),
        }
    }
}

type Map = Vec<Vec<Tile>>;
type Antennas = HashMap<char, Vec<Vector2>>;

fn read_input() -> (Map, Antennas) {
    let mut map: Map = Vec::new();
    let mut antennas: Antennas = HashMap::new();

    for (y, row) in std::io::stdin()
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| line.trim().to_string())
        .enumerate()
    {
        let mut tile_row = Vec::new();
        for (x, tile_char) in row.chars().enumerate() {
            tile_row.push(match tile_char {
                '.' => Tile::Space,
                antenna_name => {
                    let antenna = antennas.entry(antenna_name).or_insert(Vec::new());
                    antenna.push(Vector2(x as isize, y as isize));
                    Tile::Antenna(antenna_name, 0)
                }
            });
        }
        map.push(tile_row);
    }

    (map, antennas)
}

fn get_tile_mut<'a>(pos: &Vector2, map: &'a mut Map) -> Option<&'a mut Tile> {
    if pos.1 < 0 || pos.0 < 0 {
        None
    } else if let Some(row) = map.get_mut(pos.1 as usize) {
        row.get_mut(pos.0 as usize)
    } else {
        None
    }
}

fn print_map(map: &Map, a: &Vector2, b: &Vector2, c: &Vector2) -> std::io::Result<()> {
    let mut stdout = std::io::stdout();

    for (y, row) in map.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            let position = Vector2(x as isize, y as isize);
            stdout.queue(cursor::MoveTo(x as u16, y as u16))?;

            match tile {
                Tile::Space => stdout.queue(style::PrintStyledContent(".".dark_grey())),
                Tile::Antinode => {
                    if *c == position {
                        stdout.queue(style::PrintStyledContent("#".red()))
                    } else {
                        stdout.queue(style::PrintStyledContent("#".dark_red()))
                    }
                }
                Tile::Antenna(name, count) => {
                    if *a == position || *b == position {
                        stdout.queue(style::PrintStyledContent(name.green()))
                    } else if *count > 0 {
                        stdout.queue(style::PrintStyledContent(name.dark_red()))
                    } else {
                        stdout.queue(style::PrintStyledContent(name.grey()))
                    }
                }
            }?;
        }
    }
    println!("\n");
    stdout.flush()?;

    sleep(Duration::from_millis(50));

    Ok(())
}

fn add_antinode<'a>(position: &Vector2, map: &'a mut Map) -> Result<(), ()> {
    match get_tile_mut(position, map) {
        Some(ref mut tile) => match tile {
            Tile::Space => {
                **tile = Tile::Antinode;
                Ok(())
            }
            Tile::Antenna(_, ref mut count) => {
                *count += 1;
                Ok(())
            }
            Tile::Antinode => Ok(()),
        },
        None => Err(()),
    }
}

fn part1() -> Option<isize> {
    let (mut map, antennas) = read_input();
    io::stdout()
        .execute(terminal::Clear(terminal::ClearType::All))
        .unwrap();

    for positions in antennas.values() {
        let positions_clone = positions.clone();

        for (i, a) in positions.iter().enumerate() {
            for b in &positions_clone[i + 1..] {
                let c_direction = a.get_direction(b);
                let normal = c_direction.normalize();

                let c_1 = b.add(&normal);
                match add_antinode(&c_1, &mut map) {
                    Ok(_) => {
                        print_map(&map, a, b, &c_1).unwrap();
                    }
                    Err(_) => {}
                }

                let c_2 = a.sub(&normal);
                match add_antinode(&c_2, &mut map) {
                    Ok(_) => {
                        print_map(&map, a, b, &c_2).unwrap();
                    }
                    Err(_) => {}
                }
            }
        }
    }

    let mut res = 0;
    for row in map.iter() {
        for tile in row.iter() {
            match tile {
                Tile::Antinode => {
                    res += 1;
                }
                Tile::Antenna(_, count) => {
                    if *count > 0 {
                        res += 1;
                    }
                }
                _ => {}
            }
        }
    }
    println!("");
    Some(res)
}

fn part2() -> Option<isize> {
    let (mut map, antennas) = read_input();

    io::stdout()
        .execute(terminal::Clear(terminal::ClearType::All))
        .unwrap();

    for positions in antennas.values() {
        let positions_clone = positions.clone();

        for (i, a) in positions.iter().enumerate() {
            for b in &positions_clone[i + 1..] {
                let direction = a.get_direction(b);
                let normal = direction.normalize();

                let mut c_1 = b.add(&normal);
                while let Ok(_) = add_antinode(&c_1, &mut map) {
                    c_1 = c_1.add(&normal);
                }

                let mut c_2 = a.sub(&normal);
                while let Ok(_) = add_antinode(&c_2, &mut map) {
                    c_2 = c_2.sub(&normal);
                }

                print_map(&map, a, b, &Vector2::zero()).unwrap();
            }
        }
    }

    let mut res = 0;
    for row in map.iter() {
        for tile in row.iter() {
            match tile {
                Tile::Antinode | Tile::Antenna(_, _) => {
                    res += 1;
                }
                _ => {}
            }
        }
    }

    Some(res)
}

fn main() {
    println!("--- Day 8: Resonant Collinearity ---");
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
