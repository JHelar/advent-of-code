use std::{
    collections::HashMap,
    fmt::{write, Display},
};

type Vector = (u32, u32);

#[derive(Debug)]
struct Claim {
    id: usize,
    position: Vector,
    size: Vector,
}

#[derive(Debug)]
struct Fabric(HashMap<Vector, Vec<usize>>);

impl Fabric {
    fn new() -> Self {
        Self {
            0: Default::default(),
        }
    }

    fn cut(&mut self, claim: &Claim) {
        let x_min = claim.position.0;
        let x_max = x_min + claim.size.0;

        let y_min = claim.position.1;
        let y_max = y_min + claim.size.1;

        let positions = (x_min..x_max).flat_map(|x| (y_min..y_max).map(move |y| (x, y)));

        for position in positions {
            if let Some(claims) = self.0.get_mut(&position) {
                claims.push(claim.id);
            } else {
                self.0.insert(position, vec![claim.id]);
            }
        }
    }

    fn is_alone(&self, claim: &Claim) -> bool {
        self.0
            .values()
            .filter(|claims| claims.contains(&claim.id))
            .all(|claims| claims.len() == 1)
    }

    fn get_count(&self) -> u32 {
        self.0
            .values()
            .into_iter()
            .filter(|claims| claims.len() > 1)
            .count() as u32
    }
}

impl Display for Fabric {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let min_x = *self.0.keys().map(|(x, _)| x).min().unwrap();
        let max_x = *self.0.keys().map(|(x, _)| x).max().unwrap();

        let min_y = *self.0.keys().map(|(_, y)| y).min().unwrap();
        let max_y = *self.0.keys().map(|(_, y)| y).max().unwrap();

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if let Some(claims) = self.0.get(&(x, y)) {
                    let tile = if claims.len() > 1 { 'X' } else { '.' };
                    write!(f, "{tile}").unwrap();
                } else {
                    write!(f, " ").unwrap();
                }
            }
            write!(f, "\n").unwrap();
        }

        Ok(())
    }
}

impl Claim {
    fn from_str(id: usize, str: &str) -> Self {
        let (coordinate_str, size_str) = str
            .split_once('@')
            .unwrap()
            .1
            .trim()
            .split_once(':')
            .unwrap();

        let (x_str, y_str) = coordinate_str.split_once(',').unwrap();
        let (width_str, height_str) = size_str.trim().split_once('x').unwrap();

        Self {
            id: id + 1,
            position: (x_str.parse().unwrap(), y_str.parse().unwrap()),
            size: (width_str.parse().unwrap(), height_str.parse().unwrap()),
        }
    }
}

fn read_input() -> Vec<Claim> {
    std::io::stdin()
        .lines()
        .filter_map(|line| line.ok())
        .enumerate()
        .map(|(id, line)| Claim::from_str(id, line.trim()))
        .collect()
}

fn part1() -> Option<u32> {
    let claims = read_input();
    let mut fabric = Fabric::new();

    for claim in claims.iter() {
        fabric.cut(claim);
    }

    let count = fabric.get_count();
    println!("{fabric}");
    Some(count)
}

fn part2() -> Option<u32> {
    let claims = read_input();
    let mut fabric = Fabric::new();

    for claim in claims.iter() {
        fabric.cut(claim);
    }

    if let Some(claim) = claims.iter().find(|claim| fabric.is_alone(claim)) {
        Some(claim.id as u32)
    } else {
        None
    }
}

fn main() {
    println!("--- Day 3: No Matter How You Slice It ---");
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
