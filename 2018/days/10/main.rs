use std::fmt::Display;

#[derive(Debug)]
struct Vector2 {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Point {
    position: Vector2,
    velocity: Vector2,
}

#[derive(Debug)]
struct SkyMap {
    points: Vec<Point>,
    min: Vector2,
    max: Vector2,
}

impl SkyMap {
    fn new(points: Vec<Point>) -> Self {
        let x_min = points.iter().map(|point| point.position.x).min().unwrap();
        let x_max = points.iter().map(|point| point.position.x).max().unwrap();

        let y_min = points.iter().map(|point| point.position.y).min().unwrap();
        let y_max = points.iter().map(|point| point.position.y).max().unwrap();

        Self {
            points,
            min: Vector2 { x: x_min, y: y_min },
            max: Vector2 { x: x_max, y: y_max },
        }
    }

    fn forwards(&mut self, amount: i32) {
        for point in self.points.iter_mut() {
            point.forwards(amount);
        }

        self.calc_size();
    }

    fn calc_size(&mut self) {
        self.min.x = self
            .points
            .iter()
            .map(|point| point.position.x)
            .min()
            .unwrap();

        self.max.x = self
            .points
            .iter()
            .map(|point| point.position.x)
            .max()
            .unwrap();

        self.min.y = self
            .points
            .iter()
            .map(|point| point.position.y)
            .min()
            .unwrap();

        self.max.y = self
            .points
            .iter()
            .map(|point| point.position.y)
            .max()
            .unwrap();
    }

    fn get_size(&self) -> Vector2 {
        let width = self.max.x - self.min.x;
        let height = self.max.y - self.min.y;

        Vector2 {
            x: width,
            y: height,
        }
    }

    fn backwards(&mut self, amount: i32) {
        for point in self.points.iter_mut() {
            point.backwards(amount);
        }

        self.calc_size();
    }
}

impl Display for SkyMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in self.min.y..self.max.y {
            let mut line = String::new();
            for x in self.min.x..self.max.x {
                let point = self
                    .points
                    .iter()
                    .any(|point| point.position.x == x && point.position.y == y);
                if point {
                    line.push('#');
                } else {
                    line.push('.');
                }
            }
            write!(f, "{line}\n")?;
        }
        Ok(())
    }
}

impl Point {
    fn from_str(str: &str) -> Self {
        let normalized = str.replace("position=", "").replace("velocity=", "");

        let (position_str, velocity_str) = normalized.split_once("> <").unwrap();

        Self {
            position: Vector2::from_str(position_str),
            velocity: Vector2::from_str(velocity_str),
        }
    }

    fn forwards(&mut self, amount: i32) {
        self.position = self.position.add(&self.velocity.multiply(amount));
    }

    fn backwards(&mut self, amount: i32) {
        self.position = self.position.sub(&self.velocity.multiply(amount));
    }
}

impl Vector2 {
    fn from_str(str: &str) -> Self {
        let (x_str, y_str) = str.split_once(", ").unwrap();
        let x = x_str.replace("<", "").trim_start().parse().unwrap();
        let y = y_str.replace(">", "").trim_start().parse().unwrap();

        Self { x, y }
    }

    fn add(&self, other: &Vector2) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    fn sub(&self, other: &Vector2) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    fn multiply(&self, scalar: i32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

fn read_input() -> Vec<Point> {
    std::io::stdin()
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| Point::from_str(line.trim()))
        .collect()
}

fn part1() -> Option<u32> {
    let points = read_input();
    let mut map = SkyMap::new(points);
    let mut best_size = map.get_size();
    for _ in 0..100000 {
        map.forwards(1);
        let new_size = map.get_size();
        if new_size.x > best_size.x {
            map.backwards(1);
            break;
        }
        best_size = new_size;
    }
    println!("{map}");
    None
}

fn part2() -> Option<u32> {
    let points = read_input();
    let mut map = SkyMap::new(points);
    let mut best_size = map.get_size();
    for seconds in 0..100000 {
        map.forwards(1);
        let new_size = map.get_size();
        if new_size.x > best_size.x {
            return Some(seconds as u32);
        }
        best_size = new_size;
    }
    println!("{map}");
    None
}

fn main() {
    println!("--- Day 10: The Stars Align ---");
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
