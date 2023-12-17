use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

type Position = (i32, i32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    fn get_delta(&self) -> Position {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Right => (1, 0),
            Direction::Left => (-1, 0),
        }
    }

    fn get_neighbours(&self) -> Vec<Direction> {
        match self {
            Direction::Up => vec![Direction::Up, Direction::Left, Direction::Right],
            Direction::Down => vec![Direction::Down, Direction::Left, Direction::Right],
            Direction::Right => vec![Direction::Right, Direction::Up, Direction::Down],
            Direction::Left => vec![Direction::Left, Direction::Up, Direction::Down],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Block {
    heat_loss: i32,
    position: Position,
    path_part: Option<Direction>,
}

fn read_input() -> (HashMap<Position, Block>, i32, i32) {
    let mut map = HashMap::new();
    let mut x_max = 0;
    let mut y_max = 0;
    std::io::stdin()
        .lines()
        .filter_map(|line| line.ok())
        .enumerate()
        .for_each(|(y, line)| {
            line.trim()
                .chars()
                .filter_map(|c| c.to_string().parse::<i32>().ok())
                .enumerate()
                .for_each(|(x, heat_loss)| {
                    map.insert(
                        (x as i32, y as i32),
                        Block {
                            position: (x as i32, y as i32),
                            heat_loss,
                            path_part: None,
                        },
                    );
                    x_max = x as i32;
                });
            y_max = y as i32;
        });
    (map, x_max, y_max)
}

fn get_neighbours(
    (x, y): Position,
    direction: Direction,
    map: &HashMap<Position, Block>,
) -> Vec<(Direction, Block)> {
    direction
        .get_neighbours()
        .iter()
        .map(|direction| (direction, direction.get_delta()))
        .map(|(direction, (dx, dy))| (direction, map.get(&(dx + x, dy + y))))
        .filter(|(_, block)| block.is_some())
        .map(|(direction, block)| (*direction, *block.unwrap()))
        .collect()
}

fn find_path(
    start_block: &Block,
    end_block: &Block,
    max_line_length: i32,
    min_line_length: i32,
    ultra: bool,
    map: &mut HashMap<Position, Block>,
    x_max: i32,
    y_max: i32,
) -> Option<u32> {
    let mut visit = Vec::new();
    visit.push((start_block.clone(), (Direction::Right, -1_i32), 0));
    visit.push((start_block.clone(), (Direction::Down, -1_i32), 0));

    let mut path = HashMap::new();
    let mut costs = HashMap::new();
    let mut visited = HashSet::new();

    costs.insert((start_block.position, Direction::Right, -1), 0);
    costs.insert((start_block.position, Direction::Down, -1), 0);

    while let Some((current_block, (direction, direction_count), current_g)) = visit.pop() {
        if ultra && current_block == *end_block && direction_count < min_line_length {
            continue;
        }

        if current_block == *end_block {
            let mut new_path = Vec::new();
            let mut current_direction = direction;
            let mut current_position = current_block.position;
            let mut current_count = direction_count;
            while current_position != start_block.position {
                new_path.push((current_position, current_direction));
                if let Some(&(prev_pos, prev_direction, prev_count)) =
                    path.get(&(current_position, current_direction, current_count))
                {
                    current_position = prev_pos;
                    current_direction = prev_direction;
                    current_count = prev_count;
                } else {
                    break;
                }
            }
            new_path.reverse();
            print_map_path(map, x_max, y_max, &new_path);
            return Some(current_g as u32);
        }

        if !visited.insert((current_block.position, direction, direction_count)) {
            continue;
        }

        let neighbours = get_neighbours(current_block.position, direction, map);
        neighbours
            .iter()
            .filter(|(neighbour_direction, _)| {
                if !ultra {
                    return true;
                }
                // Try turn
                if *neighbour_direction != direction {
                    return direction_count >= min_line_length;
                }
                return true;

            })
            .for_each(|&(neighbour_direction, neighbour)| {
                let same_direction = neighbour_direction == direction;
                let next_direction_count = if same_direction {
                    direction_count + 1
                } else {
                    0
                };

                if next_direction_count < max_line_length && same_direction || !same_direction
                {
                    let g = current_g + neighbour.heat_loss;
                    let previous_score = costs
                        .get(&(
                            neighbour.position,
                            neighbour_direction,
                            next_direction_count,
                        ))
                        .unwrap_or(&i32::MAX);

                    if g < *previous_score {
                        let f = g;

                        path.insert(
                            (
                                neighbour.position,
                                neighbour_direction,
                                next_direction_count,
                            ),
                            (current_block.position, direction, direction_count),
                        );
                        costs.insert(
                            (
                                neighbour.position,
                                neighbour_direction,
                                next_direction_count,
                            ),
                            f,
                        );

                        let visit_node =
                            (neighbour, (neighbour_direction, next_direction_count), f);
                        if !visit.contains(&visit_node) {
                            visit.push(visit_node);
                        }
                    }
                }
            });

        visit.sort_by(|a, b| {
            let a_f = a.2;
            let b_f = b.2;
            b_f.cmp(&a_f)
        });
    }
    None
}

fn print_map_path(
    map: &mut HashMap<Position, Block>,
    x_max: i32,
    y_max: i32,
    path: &Vec<(Position, Direction)>,
) {
    for (position, direction) in path {
        let block = map.get_mut(position).unwrap();
        block.path_part = Some(*direction);
    }

    for y in 0..=y_max {
        for x in 0..=x_max {
            let block = map.get(&(x, y)).unwrap();
            match block {
                Block {
                    heat_loss: _,
                    position: _,
                    path_part: Some(direction),
                } => match direction {
                    Direction::Down => print!("v"),
                    Direction::Up => print!("^"),
                    Direction::Right => print!(">"),
                    Direction::Left => print!("<"),
                },
                Block {
                    heat_loss,
                    position: _,
                    path_part: None,
                } => {
                    print!("{heat_loss}");
                }
            }
        }
        print!("\n");
    }
}

fn part1() -> Option<u32> {
    let (mut map, x_max, y_max) = read_input();
    let start_block = map.get(&(0, 0)).unwrap().clone();
    let end_block = map.get(&(x_max, y_max)).unwrap().clone();

    if let Some(result) = find_path(&start_block, &end_block, 3, 0, false, &mut map, x_max, y_max) {
        Some(result)
    } else {
        None
    }
}

fn part2() -> Option<u32> {
    let (mut map, x_max, y_max) = read_input();
    let start_block = map.get(&(0, 0)).unwrap().clone();
    let end_block = map.get(&(x_max, y_max)).unwrap().clone();

    if let Some(result) = find_path(&start_block, &end_block, 10, 3, true, &mut map, x_max, y_max) {
        Some(result)
    } else {
        None
    }
}

fn main() {
    println!("--- Day 17: Clumsy Crucible ---");
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
