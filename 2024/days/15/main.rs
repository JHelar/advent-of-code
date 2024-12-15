use std::fmt::Display;

use vector2::{Vector2, DOWN, LEFT, RIGHT, UP, ZERO};

const SMALL_BOX: Vector2 = ZERO;
const LEFT_SIDE: Vector2 = RIGHT;
const RIGHT_SIDE: Vector2 = LEFT;

mod vector2;

#[derive(Debug, Clone)]
enum Tile {
    Wall,
    Empty,
    Box(Vector2),
    Robot,
}

#[derive(Debug)]
struct Warehouse {
    map: Vec<Tile>,
    size: Vector2,
}

impl Warehouse {
    fn get_tile_index(&self, position: &Vector2) -> usize {
        (position.0 + position.1 * self.size.0) as usize
    }

    fn get_tile(&self, position: &Vector2) -> Option<&Tile> {
        let index = self.get_tile_index(position);
        self.map.get(index)
    }

    fn get_tile_mut(&mut self, position: &Vector2) -> Option<&mut Tile> {
        let index = self.get_tile_index(position);
        self.map.get_mut(index)
    }

    fn run(&mut self, robot: &Vector2, direction: &Vector2) -> Vector2 {
        let prev_map = self.map.clone();
        match self.step((robot, Tile::Robot), direction) {
            Ok(new_position) => new_position,
            Err(_) => {
                self.map = prev_map;
                robot.clone()
            }
        }
    }

    fn step(&mut self, entity: (&Vector2, Tile), direction: &Vector2) -> Result<Vector2, ()> {
        let new_position = entity.0.add(direction);
        match self.get_tile(&new_position).cloned() {
            Some(tile) => match tile {
                Tile::Empty => {
                    *self.get_tile_mut(entity.0).unwrap() = Tile::Empty;
                    *self.get_tile_mut(&new_position).unwrap() = entity.1;
                    Ok(new_position)
                }
                Tile::Box(side) if *direction == LEFT || *direction == RIGHT || side == SMALL_BOX => {
                    match self.step((&new_position, Tile::Box(side)), direction) {
                        Ok(_) => {
                            *self.get_tile_mut(entity.0).unwrap() = Tile::Empty;
                            *self.get_tile_mut(&new_position).unwrap() = entity.1;

                            Ok(new_position)
                        }
                        _ => Err(()),
                    }
                }
                Tile::Box(side) => {
                    let one = self.step((&new_position, Tile::Box(side)), direction);

                    let opposite = new_position.add(&side);
                    let another = self.step((&opposite, Tile::Box(side.mul_scalar(-1))), direction);

                    match (one, another) {
                        (Ok(_), Ok(_)) => {
                            *self.get_tile_mut(entity.0).unwrap() = Tile::Empty;
                            *self.get_tile_mut(&new_position).unwrap() = entity.1;

                            Ok(new_position)
                        }
                        _ => Err(()),
                    }
                }
                Tile::Robot => panic!("Moving into a robot is not possible"),
                Tile::Wall => Err(()),
            },
            None => panic!("Moving outside the map"),
        }
    }

    fn get_gps_state(&self) -> isize {
        let mut gps = 0;
        for x in 0..self.size.0 {
            for y in 0..self.size.1 {
                gps += match self.get_tile(&Vector2(x, y)).unwrap() {
                    Tile::Box(side) if *side == SMALL_BOX => y * 100 + x,
                    Tile::Box(side) if *side == LEFT_SIDE => y * 100 + x,
                    _ => 0,
                }
            }
        }

        gps
    }
}

impl Display for Warehouse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.size.1 {
            for x in 0..self.size.0 {
                match self.get_tile(&Vector2(x, y)) {
                    Some(tile) => match tile {
                        Tile::Box(side) => match *side {
                            LEFT_SIDE => write!(f, "["),
                            RIGHT_SIDE => write!(f, "]"),
                            SMALL_BOX => write!(f, "O"),
                            _ => panic!("Bad box"),
                        },
                        Tile::Empty => write!(f, "."),
                        Tile::Robot => write!(f, "@"),
                        Tile::Wall => write!(f, "#"),
                    },
                    None => panic!("ooops"),
                }?;
            }
            write!(f, "\n")?;
        }
        write!(f, "\n")
    }
}

fn read_input(double_size: bool) -> (Warehouse, Vector2, Vec<Vector2>) {
    let mut read_map = true;
    let mut map_size = Vector2::zero();
    let mut map = Vec::new();
    let mut directions = Vec::new();

    for row in std::io::stdin()
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| line.trim().to_string())
    {
        if row.is_empty() {
            read_map = false;
            if double_size {
                map_size = map_size.mul(&Vector2(2, 1));
            }
        }
        if read_map {
            map_size.1 = row.len() as isize;

            for (x, tile_char) in row.chars().enumerate() {
                map_size.0 = map_size.0.max((x + 1) as isize);
                let tile = match tile_char {
                    '#' => Tile::Wall,
                    '.' => Tile::Empty,
                    'O' => Tile::Box(SMALL_BOX),
                    '@' => Tile::Robot,
                    c => panic!("Unknown tile: {c}"),
                };

                if double_size {
                    match tile {
                        Tile::Wall => {
                            map.push(Tile::Wall);
                            map.push(Tile::Wall);
                        }
                        Tile::Empty => {
                            map.push(Tile::Empty);
                            map.push(Tile::Empty);
                        }
                        Tile::Box(_) => {
                            map.push(Tile::Box(LEFT_SIDE));
                            map.push(Tile::Box(RIGHT_SIDE));
                        }
                        Tile::Robot => {
                            map.push(Tile::Robot);
                            map.push(Tile::Empty);
                        }
                    }
                } else {
                    map.push(tile.clone());
                }
            }
        } else {
            for direction in row.chars().map(|direction_char| match direction_char {
                '<' => LEFT,
                'v' => DOWN,
                '^' => UP,
                '>' => RIGHT,
                c => panic!("Unknown direction: {c}"),
            }) {
                directions.push(direction);
            }
        }
    }

    let warehouse = Warehouse {
        map,
        size: map_size,
    };

    let mut robot_position = ZERO;
    'outer: for x in 0..map_size.0 {
        for y in 0..map_size.1 {
            let position = Vector2(x, y);
            let tile = warehouse.get_tile(&position).unwrap();
            if matches!(tile, Tile::Robot) {
                robot_position = position;
                break 'outer;
            }
        }
    }

    (warehouse, robot_position, directions)
}

fn part1() -> Option<isize> {
    let (mut warehouse, mut robot, directions) = read_input(false);

    println!("{warehouse}");
    for direction in directions.iter() {
        robot = warehouse.run(&robot, direction);
        println!("{warehouse}");
    }
    Some(warehouse.get_gps_state())
}

fn part2() -> Option<isize> {
    let (mut warehouse, mut robot, directions) = read_input(true);

    println!("{warehouse}");
    for direction in directions.iter() {
        robot = warehouse.run(&robot, direction);
        println!("{warehouse}");
    }

    Some(warehouse.get_gps_state())
}

fn main() {
    println!("--- Day 15: Warehouse Woes ---");
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
