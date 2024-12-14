use std::fmt::Display;

use hashbrown::HashSet;
use vector2::{Vector2, DOWN, RIGHT};

mod vector2;

#[derive(Debug)]
struct Robot {
    id: usize,
    position: Vector2,
    velocity: Vector2,
}

impl Robot {
    fn from_str(str: &str, id: usize) -> Self {
        let (pos_str, vel_str) = str.split_once(" ").unwrap();

        let pos = pos_str.replace("p=", "");
        let (x, y) = pos.split_once(",").unwrap();

        let vel = vel_str.replace("v=", "");
        let (vx, vy) = vel.split_once(",").unwrap();

        Self {
            id,
            position: Vector2(x.parse().unwrap(), y.parse().unwrap()),
            velocity: Vector2(vx.parse().unwrap(), vy.parse().unwrap()),
        }
    }
}

#[derive(Debug, Clone)]
struct Map {
    tiles: Vec<HashSet<usize>>,
    size: Vector2,
}

impl std::hash::Hash for Map {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let tiles_vec = self.tiles.iter().map(|t| t.len()).collect::<Vec<usize>>();
        tiles_vec.hash(state);
    }
}

impl Map {
    fn new(size: Vector2) -> Self {
        let mut tiles = Vec::with_capacity((size.0 * size.1) as usize);
        for _ in 0..tiles.capacity() {
            tiles.push(HashSet::new());
        }

        Self { tiles, size }
    }

    fn set_robots(&mut self, robots: &Vec<Robot>) {
        for robot in robots.iter() {
            self.get_tile_mut(&robot.position).unwrap().insert(robot.id);
        }
    }

    fn get_wrapped_position(&self, position: &Vector2) -> Vector2 {
        let x = match position.0 {
            x if x < 0 => self.size.0 + x,
            x if x >= self.size.0 => x % self.size.0,
            x => x,
        };

        let y = match position.1 {
            y if y < 0 => self.size.1 + y,
            y if y >= self.size.1 => y % self.size.1,
            y => y,
        };

        Vector2(x, y)
    }

    fn get_tile_index(&self, position: &Vector2) -> usize {
        (position.0 + position.1 * self.size.0) as usize
    }

    fn get_tile(&self, position: &Vector2) -> Option<&HashSet<usize>> {
        let index = self.get_tile_index(position);
        self.tiles.get(index)
    }

    fn get_tile_mut(&mut self, position: &Vector2) -> Option<&mut HashSet<usize>> {
        let index = self.get_tile_index(position);
        self.tiles.get_mut(index)
    }

    fn get_quadrant_values(&self) -> Vec<usize> {
        let width = self.size.0 / 2;
        let height = self.size.1 / 2;

        let mut quadrants = Vec::new();
        for quadrant in [Vector2::zero(), RIGHT, DOWN, Vector2(1, 1)] {
            let mut value = 0;
            for y in
                (quadrant.1 + quadrant.1 * height)..(height + (quadrant.1 + quadrant.1 * height))
            {
                for x in
                    (quadrant.0 + quadrant.0 * width)..(width + (quadrant.0 + quadrant.0 * width))
                {
                    let tile = self.get_tile(&Vector2(x, y)).unwrap();
                    value += tile.len();
                }
            }
            quadrants.push(value);
        }

        quadrants
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.size.1 {
            for x in 0..self.size.0 {
                match self.get_tile(&Vector2(x, y)) {
                    Some(tile) if tile.len() == 0 => write!(f, " "),
                    Some(tile) => write!(f, "{}", tile.len()),
                    None => panic!("ooops"),
                }?;
            }
            write!(f, "\n")?;
        }
        write!(f, "\n")
    }
}

impl PartialEq for Map {
    fn eq(&self, other: &Self) -> bool {
        self.tiles == other.tiles
    }
}

impl Eq for Map {}

fn step(map: &mut Map, robot: &mut Robot) {
    map.get_tile_mut(&robot.position).unwrap().remove(&robot.id);
    let new_pos = map.get_wrapped_position(&robot.position.add(&robot.velocity));

    robot.position = new_pos;
    map.get_tile_mut(&robot.position).unwrap().insert(robot.id);
}

fn read_input() -> Vec<Robot> {
    let mut robot_id = 0;
    std::io::stdin()
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            let robot = Robot::from_str(line.trim(), robot_id);
            robot_id += 1;

            robot
        })
        .collect()
}

fn part1() -> Option<isize> {
    let mut robots = read_input();
    let mut map = Map::new(Vector2(101, 103));
    map.set_robots(&robots);

    for _ in 0..100 {
        for robot in robots.iter_mut() {
            step(&mut map, robot);
        }
    }
    println!("Sec: 100\n{map}");
    let sum = map
        .get_quadrant_values()
        .into_iter()
        .fold(1, |sum, val| sum * val) as isize;
    Some(sum)
}

const TREE_SEC: isize = 6512;
const CYCLE_LENGTH: isize = 10403;

fn part2() -> Option<isize> {
    let mut robots = read_input();
    let mut map = Map::new(Vector2(101, 103));
    map.set_robots(&robots);

    // Stared at the screen for 0 to CYCLE_LENGTH...
    for sec in 0..CYCLE_LENGTH {
        if sec == TREE_SEC {
            println!("Sec: {sec}\n{map}");
            break;
        }
        for robot in robots.iter_mut() {
            step(&mut map, robot);
        }
    }

    Some(TREE_SEC)
}

fn main() {
    println!("--- Day 14: Restroom Redoubt ---");
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
