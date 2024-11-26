mod vector2;

use hashbrown::{HashMap, HashSet};

use std::{collections::VecDeque, fmt::Display};

use vector2::Vector2;

const DOWN: Vector2 = Vector2(0, 1);
const LEFT: Vector2 = Vector2(-1, 0);
const RIGHT: Vector2 = Vector2(1, 0);

#[derive(Debug)]
enum TileType {
    Wall,
    Sand,
    Water,
}

#[derive(Debug, Clone, Copy)]
struct Wall {
    start: Vector2,
    end: Vector2,
}

impl Wall {
    fn from_str(str: &str) -> Self {
        let (static_axis, range_axis) = str.split_once(", ").unwrap();

        // Parse floor
        if static_axis.starts_with("y") {
            let y = static_axis.replace("y=", "").parse::<isize>().unwrap();
            let x_range = range_axis.replace("x=", "");
            let (x_start_str, x_end_str) = x_range.split_once("..").unwrap();

            Self {
                start: Vector2::new(x_start_str.parse().unwrap(), y),
                end: Vector2::new(x_end_str.parse().unwrap(), y),
            }
        } else {
            let x = static_axis.replace("x=", "").parse::<isize>().unwrap();
            let y_range = range_axis.replace("y=", "");
            let (y_start_str, y_end_str) = y_range.split_once("..").unwrap();

            Self {
                start: Vector2::new(x, y_start_str.parse().unwrap()),
                end: Vector2::new(x, y_end_str.parse().unwrap()),
            }
        }
    }

    fn hit(&self, point: &Vector2) -> bool {
        let hit_x = self.start.0 <= point.0 && point.0 <= self.end.0;
        let hit_y = self.start.1 <= point.1 && point.1 <= self.end.1;
        hit_x && hit_y
    }
}

#[derive(Debug)]
struct Reservoir {
    start: Vector2,
    end: Vector2,
    walls: Vec<Wall>,
    walls_map: HashMap<Vector2, Option<Wall>>,
    water: HashSet<Vector2>,
    wet_sand: HashSet<Vector2>,
}

impl Reservoir {
    fn new(walls: Vec<Wall>) -> Self {
        let min_x = walls.iter().map(|wall| wall.start.0).min().unwrap();
        let min_y = walls.iter().map(|wall| wall.start.1).min().unwrap();
        let max_x = walls.iter().map(|wall| wall.end.0).max().unwrap();
        let max_y = walls.iter().map(|wall| wall.end.1).max().unwrap();

        Self {
            walls,
            water: HashSet::new(),
            wet_sand: HashSet::new(),
            walls_map: HashMap::new(),
            start: Vector2::new(min_x, min_y),
            end: Vector2::new(max_x, max_y),
        }
    }

    fn drop_water(&mut self, spring: &Vector2) -> bool {
        let mut visit = VecDeque::from([*spring]);
        let mut visited = HashSet::new();

        let mut placed_water = false;

        while let Some(source) = visit.pop_front() {
            visited.insert(source.clone());
            if source.1 > self.end.1 {
                continue;
            }

            let down_point = source.add(&DOWN);
            let source_tile = self.get_tile_type(&source);
            let down_tile = self.get_tile_type(&down_point);

            match (source_tile, down_tile) {
                (TileType::Sand, TileType::Sand) => {
                    // Drop
                    visit.push_back(down_point);
                    self.wet_sand.insert(source);
                }
                (TileType::Sand, TileType::Wall | TileType::Water) => {
                    let is_contained =
                        self.hits_wall(&source, &LEFT) && self.hits_wall(&source, &RIGHT);
                    if is_contained {
                        placed_water = true;
                        self.water.insert(source);
                        self.wet_sand.remove(&source);
                    } else {
                        self.wet_sand.insert(source);
                    }

                    let left = source.add(&LEFT);
                    if !visited.contains(&left) {
                        visit.push_back(left);
                    }
                    let right = source.add(&RIGHT);
                    if !visited.contains(&right) {
                        visit.push_back(right);
                    }
                }
                _ => {}
            }
        }
        placed_water
    }

    fn hits_wall(&mut self, point: &Vector2, direction: &Vector2) -> bool {
        match (
            self.get_tile_type(point),
            self.get_tile_type(&point.add(&DOWN)),
        ) {
            (TileType::Sand, TileType::Sand) => false,
            (TileType::Wall, TileType::Wall) => true,
            _ => self.hits_wall(&point.add(direction), direction),
        }
    }

    fn get_tile_type(&mut self, point: &Vector2) -> TileType {
        let hit_wall = self
            .walls_map
            .entry(*point)
            .or_insert(self.walls.iter().find(|&wall| wall.hit(point)).copied());

        let hit_water = self.water.contains(point);

        if hit_wall.is_some() {
            TileType::Wall
        } else if hit_water {
            TileType::Water
        } else {
            TileType::Sand
        }
    }
}

impl Display for Reservoir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in (self.start.1 - 1)..=(self.end.1 + 1) {
            for x in (self.start.0 - 1)..=(self.end.0 + 1) {
                let point = Vector2::new(x, y);

                if self.walls.iter().any(|wall| wall.hit(&point)) {
                    write!(f, "#")?;
                } else if self.wet_sand.contains(&point) {
                    write!(f, "|")?;
                } else if self.water.contains(&point) {
                    write!(f, "~")?;
                } else {
                    write!(f, ".")?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn read_input() -> Reservoir {
    let walls = std::io::stdin()
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| Wall::from_str(&line))
        .collect();

    Reservoir::new(walls)
}

fn part1() -> Option<usize> {
    let mut reservoir = read_input();
    let spring = Vector2::new(500, 0);

    while reservoir.drop_water(&spring) {}

    let mut points: HashSet<&Vector2> = reservoir.water.union(&reservoir.wet_sand).collect();
    points = points
        .into_iter()
        .filter(|point| !(point.1 > reservoir.end.1 || point.1 < reservoir.start.1))
        .collect();
    Some(points.len())
}

fn part2() -> Option<usize> {
    let mut reservoir = read_input();
    let spring = Vector2::new(500, 0);

    while reservoir.drop_water(&spring) {}

    let points: HashSet<Vector2> = reservoir.water
        .into_iter()
        .filter(|point| !(point.1 > reservoir.end.1 || point.1 < reservoir.start.1))
        .collect();

    Some(points.len())
}

fn main() {
    println!("--- Day 17: Reservoir Research ---");
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
