use std::collections::{HashMap, HashSet};

type Position = (i32, i32, i32);
type Map = Vec<Vec<Vec<Tile>>>;
type BrickMap = HashMap<Position, usize>;

#[derive(Debug, Clone, Copy)]
enum Tile {
    Cube(usize),
    Empty,
}

#[derive(Debug)]
struct Cube {
    x: i32,
    y: i32,
    z: i32,
}

impl Cube {
    fn get_brick(range_string: &str) -> (Vec<Cube>, i32, i32, i32) {
        let (from_string, to_string) = range_string.split_once("~").unwrap();
        let from_points = from_string
            .split(",")
            .filter_map(|pos| pos.parse::<i32>().ok());
        let to_points = to_string
            .split(",")
            .filter_map(|pos| pos.parse::<i32>().ok());
        let mut ranges = from_points.zip(to_points);

        let x_range = ranges.next().unwrap();
        let y_range = ranges.next().unwrap();
        let z_range = ranges.next().unwrap();

        let mut vectors = Vec::new();
        (z_range.0..=z_range.1).for_each(|z| {
            (y_range.0..=y_range.1).for_each(|y| {
                (x_range.0..=x_range.1).for_each(|x| vectors.push(Cube { x, y, z }));
            });
        });

        (
            vectors,
            x_range.0.max(x_range.1),
            y_range.0.max(y_range.1),
            z_range.0.max(z_range.1),
        )
    }

    fn get_pos(&self) -> Position {
        (self.x, self.y, self.z)
    }
}

fn read_input() -> (Map, usize) {
    let mut max_x = 0;
    let mut max_y = 0;
    let mut max_z = 0;

    let mut map = Vec::new();
    let mut brick_count = 0;
    let bricks: BrickMap = std::io::stdin()
        .lines()
        .filter_map(|line| line.ok())
        .enumerate()
        .flat_map(|(brick_number, line)| {
            brick_count = brick_number + 1;
            let (bricks, x, y, z) = Cube::get_brick(line.trim());
            max_x = max_x.max(x);
            max_y = max_y.max(y);
            max_z = max_z.max(z);
            bricks
                .into_iter()
                .map(move |cube| (cube.get_pos(), brick_number))
        })
        .collect();

    for z in 0..=max_z {
        let mut depth = Vec::new();
        for y in 0..=max_y {
            let mut row = Vec::new();
            for x in 0..=max_x {
                let position = (x, y, z);
                if let Some(brick_number) = bricks.get(&position) {
                    row.push(Tile::Cube(*brick_number))
                } else {
                    row.push(Tile::Empty)
                }
            }
            depth.push(row);
        }
        map.push(depth);
    }

    (map, brick_count)
}

fn get_underside(cubes: Vec<Position>) -> Vec<Position> {
    let mut positions: HashMap<(i32, i32), Position> = HashMap::new();
    // Find cubes that have the same x and y points, take the position with lowest z point
    // Find cubes that have unique x and y points

    for (x, y, z) in cubes.into_iter() {
        let position2 = (x, y);
        if let Some(existing_cube) = positions.get_mut(&position2) {
            if z < existing_cube.2 {
                *existing_cube = (x, y, z);
            }
        } else {
            positions.insert(position2, (x, y, z));
        }
    }

    positions
        .into_iter()
        .map(|(_, position)| position)
        .collect()
}

fn get_topside(cubes: Vec<Position>) -> Vec<Position> {
    let mut positions: HashMap<(i32, i32), Position> = HashMap::new();
    // Find cubes that have the same x and y points, take the position with highest z point
    // Find cubes that have unique x and y points

    for (x, y, z) in cubes.into_iter() {
        let position2 = (x, y);
        if let Some(existing_cube) = positions.get_mut(&position2) {
            if z > existing_cube.2 {
                *existing_cube = (x, y, z);
            }
        } else {
            positions.insert(position2, (x, y, z));
        }
    }

    positions
        .into_iter()
        .map(|(_, position)| position)
        .collect()
}

fn get_bricks(map: &Map, brick_count: usize) -> Vec<Vec<(Position, usize)>> {
    let mut bricks: Vec<Vec<(Position, usize)>> = (0..brick_count).map(|_| Vec::new()).collect();
    for (z, depth) in map.iter().enumerate() {
        for (y, row) in depth.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                let position = (x as i32, y as i32, z as i32);
                match tile {
                    Tile::Cube(number) => {
                        bricks[*number].push((position, *number));
                    }
                    _ => {}
                };
            }
        }
    }

    bricks.into_iter().filter(|cubes| cubes.len() > 0).collect()
}

fn drop_bricks(map: &mut Map, brick_count: usize) -> u32 {
    let mut dropped_bricks = 0;
    let mut bricks = get_bricks(map, brick_count);

    // Sort by z, so that the lowest brick gets to fall first
    bricks.sort_by(|a, b| {
        a.iter()
            .map(|((_, _, z), _)| *z)
            .min()
            .cmp(&b.iter().map(|((_, _, z), _)| *z).min())
    });

    bricks.into_iter().for_each(|cubes| {
        let mut z_decrease = 0;
        let underside = get_underside(cubes.clone().into_iter().map(|(cube, _)| cube).collect());
        for i in 1.. {
            if underside.iter().all(|(x, y, z)| {
                let cube_z = z - i;
                if cube_z == 0 {
                    false
                } else {
                    matches!(map[cube_z as usize][*y as usize][*x as usize], Tile::Empty)
                }
            }) {
                z_decrease = i;
            } else {
                break;
            }
        }

        if z_decrease > 0 {
            dropped_bricks += 1;
            for ((x, y, z), number) in cubes.into_iter() {
                map[z as usize][y as usize][x as usize] = Tile::Empty;
                map[(z - z_decrease) as usize][y as usize][x as usize] = Tile::Cube(number);
            }
        }
    });

    dropped_bricks
}

fn part1() -> Option<u32> {
    let (mut map, brick_count) = read_input();
    drop_bricks(&mut map, brick_count);

    let bricks = get_bricks(&map, brick_count);
    let mut brick_supported_by: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut brick_supports: HashMap<usize, HashSet<usize>> = HashMap::new();

    for cubes in bricks {
        let topside = get_topside(cubes.clone().into_iter().map(|(cube, _)| cube).collect());
        let mut supports = HashSet::new();
        topside.into_iter().for_each(|(x, y, z)| {
            let top_z = (z + 1) as usize;
            if top_z < map.len() {
                match map[top_z][y as usize][x as usize] {
                    Tile::Cube(number) => {
                        supports.insert(number);

                        if let Some(supports_by) = brick_supported_by.get_mut(&number) {
                            supports_by.insert(cubes[0].1);
                        } else {
                            brick_supported_by.insert(number, {
                                let mut supports_by = HashSet::new();
                                supports_by.insert(cubes[0].1);
                                supports_by
                            });
                        }
                    }
                    _ => {}
                }
            }
        });
        brick_supports.insert(cubes[0].1, supports);
    }

    let result = (0..brick_count)
        .filter(|brick_number| {
            let supports = brick_supports.get(brick_number).unwrap();
            let can_remove = supports.iter().all(|supports_brick_number| {
                if let Some(supported_by_bricks) = brick_supported_by.get(supports_brick_number) {
                    supported_by_bricks.len() > 1
                } else {
                    false
                }
            });
            can_remove
        })
        .count();

    Some(result as u32)
}

fn part2() -> Option<u32> {
    let (mut map, brick_count) = read_input();
    drop_bricks(&mut map, brick_count);

    let bricks = get_bricks(&map, brick_count);
    let mut brick_supported_by: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut brick_supports: HashMap<usize, HashSet<usize>> = HashMap::new();

    for cubes in bricks {
        let topside = get_topside(cubes.clone().into_iter().map(|(cube, _)| cube).collect());
        let mut supports = HashSet::new();
        topside.into_iter().for_each(|(x, y, z)| {
            let top_z = (z + 1) as usize;
            if top_z < map.len() {
                match map[top_z][y as usize][x as usize] {
                    Tile::Cube(number) => {
                        supports.insert(number);

                        if let Some(supports_by) = brick_supported_by.get_mut(&number) {
                            supports_by.insert(cubes[0].1);
                        } else {
                            brick_supported_by.insert(number, {
                                let mut supports_by = HashSet::new();
                                supports_by.insert(cubes[0].1);
                                supports_by
                            });
                        }
                    }
                    _ => {}
                }
            }
        });
        brick_supports.insert(cubes[0].1, supports);
    }

    let crutial_bricks: Vec<usize> = (0..brick_count)
        .filter(|brick_number| {
            let supports = brick_supports.get(brick_number).unwrap();
            let can_remove = !supports.iter().all(|supports_brick_number| {
                if let Some(supported_by_bricks) = brick_supported_by.get(supports_brick_number) {
                    supported_by_bricks.len() > 1
                } else {
                    false
                }
            });
            can_remove
        }).collect();
    
    let mut total_drops = 0;
    for brick in crutial_bricks.into_iter() {
        let mut map_copy = map.clone();

        for depth in map_copy.iter_mut() {
          for row in depth.iter_mut() {
              for tile in row.iter_mut() {
                  match tile {
                      Tile::Cube(number) if *number == brick => {
                          *tile = Tile::Empty
                      }
                      _ => {}
                  };
              }
          }
      }

      total_drops += drop_bricks(&mut map_copy, brick_count);
    }
    
    Some(total_drops)
}

fn main() {
    println!("--- Day 22: Sand Slabs ---");
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
