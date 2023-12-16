use std::{collections::{HashMap, HashSet}, vec};

type Position = (i32, i32);

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum BeamDirection {
    Right,
    Left,
    Up,
    Down
}

#[derive(Debug, Clone, Copy)]
enum Tile {
    Empty,
    Mirror(char),
    Beam(BeamDirection),
}

fn read_input() -> (HashMap<Position, (Tile, u32)>, i32, i32) {
    let mut map: HashMap<(i32, i32), (Tile, u32)> = HashMap::new();
    let mut y_max = 0;
    let mut x_max = 0;

    std::io::stdin()
        .lines()
        .filter_map(|line| line.ok())
        .enumerate()
        .for_each(|(y, line)| {
            line.trim().chars().enumerate().for_each(|(x, char)| {
                map.insert(
                    (x as i32, y as i32),
                    match char {
                        '.' => (Tile::Empty, 0),
                        _ => (Tile::Mirror(char), 0),
                    },
                );
                x_max = x as i32;
            });
            y_max = y as i32;
        });

    (map, x_max, y_max)
}

// fn print_map(map: &mut HashMap<Position, (Tile, u32)>, with_count: bool) {
//     for y in 0..10 {
//         for x in 0..10 {
//             let (tile, visit_count) = map.get(&(x, y)).unwrap();
//             match tile {
//                 Tile::Mirror(mirror_type) => {
//                     print!("{mirror_type}");
//                 },
//                 Tile::Empty => {
//                     print!(".");
//                 },
//                 Tile::Beam(beam_direction) => {
//                     if with_count && *visit_count > 1 {
//                         print!("{visit_count}");
//                     } else {
//                         match beam_direction {
//                             BeamDirection::Up => {
//                                 print!("^");
//                             },
//                             BeamDirection::Down => {
//                                 print!("v");
//                             },
//                             BeamDirection::Left => {
//                                 print!("<");
//                             }
//                             BeamDirection::Right => {
//                                 print!(">");
//                             }
//                         }
//                     }
//                 }
//             }
//         }
//         print!("\n");
//     }
// }

fn get_new_pos((x, y): Position, beam_direction: BeamDirection) -> (Position, BeamDirection) {
    match beam_direction {
        BeamDirection::Up => ((x, y - 1), beam_direction),
        BeamDirection::Down => ((x, y + 1), beam_direction),
        BeamDirection::Left => ((x - 1, y), beam_direction),
        BeamDirection::Right => ((x + 1, y), beam_direction),
    }
}

fn search(start_position: Position, start_direction: BeamDirection, map: &mut HashMap<Position, (Tile, u32)>) {
    let mut visit = vec![(start_position, start_direction)];
    let mut visited = HashSet::new();

    while let Some((current_position, beam_direction)) = visit.pop() {
        if visited.contains(&(current_position, beam_direction)) {
            continue;
        }
        if let Some((current_tile, tile_visit_count)) = map.get_mut(&current_position) {
            match (current_tile.clone(), beam_direction) {
                (Tile::Empty, _) => {
                    *current_tile = Tile::Beam(beam_direction.clone());
                    visit.push(get_new_pos(current_position, beam_direction));
                },
    
                (Tile::Mirror('|'), BeamDirection::Left) => {
                    visit.push(get_new_pos(current_position, BeamDirection::Up));
                    visit.push(get_new_pos(current_position, BeamDirection::Down));
                },
                (Tile::Mirror('|'), BeamDirection::Right) => {
                    visit.push(get_new_pos(current_position, BeamDirection::Up));
                    visit.push(get_new_pos(current_position, BeamDirection::Down));
                },
                (Tile::Mirror('|'), _) => {
                    visit.push(get_new_pos(current_position, beam_direction));
                },
    
                (Tile::Mirror('-'), BeamDirection::Up) => {
                    visit.push(get_new_pos(current_position, BeamDirection::Left));
                    visit.push(get_new_pos(current_position, BeamDirection::Right));
                },
                (Tile::Mirror('-'), BeamDirection::Down) => {
                    visit.push(get_new_pos(current_position, BeamDirection::Left));
                    visit.push(get_new_pos(current_position, BeamDirection::Right));
                },
                (Tile::Mirror('-'), _) => {
                    visit.push(get_new_pos(current_position, beam_direction));
                },
    
                (Tile::Mirror('/'), BeamDirection::Up) => {
                    visit.push(get_new_pos(current_position, BeamDirection::Right));
                },
                (Tile::Mirror('/'), BeamDirection::Down) => {
                    visit.push(get_new_pos(current_position, BeamDirection::Left));
                },
                (Tile::Mirror('/'), BeamDirection::Left) => {
                    visit.push(get_new_pos(current_position, BeamDirection::Down));
                },
                (Tile::Mirror('/'), BeamDirection::Right) => {
                    visit.push(get_new_pos(current_position, BeamDirection::Up));
                },
    
                (Tile::Mirror('\\'), BeamDirection::Up) => {
                    visit.push(get_new_pos(current_position, BeamDirection::Left));
                },
                (Tile::Mirror('\\'), BeamDirection::Down) => {
                    visit.push(get_new_pos(current_position, BeamDirection::Right));
                },
                (Tile::Mirror('\\'), BeamDirection::Left) => {
                    visit.push(get_new_pos(current_position, BeamDirection::Up));
                },
                (Tile::Mirror('\\'), BeamDirection::Right) => {
                    visit.push(get_new_pos(current_position, BeamDirection::Down));
                },

                (Tile::Beam(_), _) => {
                    visit.push(get_new_pos(current_position, beam_direction));
                } 
                (_, _) => panic!("Combination not covered: {:?} - {:?}", current_tile, beam_direction)
            }
            *tile_visit_count += 1;
            visited.insert((current_position, beam_direction));
        }
    }
}

fn part1() -> Option<u32> {
    let (mut map, _, _) = read_input();
    search((0, 0), BeamDirection::Right, &mut map);
    let result = map.iter().filter(|(_, (_, visit_count))| *visit_count > 0).count();
    Some(result as u32)
}

fn part2() -> Option<u32> {
    let (map, x_max, y_max) = read_input();
    let mut result = 0;
    // Top
    for x in 0..=x_max {
        let mut map_copy = map.clone();
        search((x, 0), BeamDirection::Down, &mut map_copy);
        result = result.max(map_copy.iter().filter(|(_, (_, visit_count))| *visit_count > 0).count());
    }
    // Bottom
    for x in 0..=x_max {
        let mut map_copy = map.clone();
        search((x, y_max), BeamDirection::Up, &mut map_copy);
        result = result.max(map_copy.iter().filter(|(_, (_, visit_count))| *visit_count > 0).count());
    }

    // Left
    for y in 0..=y_max {
        let mut map_copy = map.clone();
        search((0, y), BeamDirection::Right, &mut map_copy);
        result = result.max(map_copy.iter().filter(|(_, (_, visit_count))| *visit_count > 0).count());
    }

    // Right
    for y in 0..=y_max {
        let mut map_copy = map.clone();
        search((x_max, y), BeamDirection::Left, &mut map_copy);
        result = result.max(map_copy.iter().filter(|(_, (_, visit_count))| *visit_count > 0).count());
    }
    Some(result as u32)
}

fn main() {
    println!("--- Day 16: The Floor Will Be Lava ---");
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
