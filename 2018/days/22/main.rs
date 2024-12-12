mod vector2;

use std::{
    borrow::BorrowMut,
    cmp::Ordering,
    collections::{BTreeSet, HashMap},
    rc::Rc,
    usize,
};

use vector2::{Vector2, DOWN, LEFT, RIGHT, UP};

type Gear = [bool; 3];

const TORCH_INDEX: usize = 0;
const CLIMBING_GEAR_INDEX: usize = 1;
const NEITHER_INDEX: usize = 2;

const TORCH_GEAR: Gear = [true, false, false];

const CLIMBING_GEAR: Gear = [false, true, false];

const NEITHER_GEAR: Gear = [false, false, true];

#[derive(Debug, Clone, Copy)]
enum Tile {
    Rock,
    Wet,
    Narrow,
}

impl Tile {
    fn from_risk_level(risk_level: isize) -> Self {
        match risk_level {
            0 => Self::Rock,
            1 => Self::Wet,
            2 => Self::Narrow,
            _ => panic!("Unknown tile type {risk_level}"),
        }
    }
}

struct Node(Vector2, Rc<usize>, [bool; 3]);

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.1.cmp(&other.1)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for Node {}

#[derive(Debug)]
struct Cave {
    depth: isize,
    target: Vector2,
    erosion_levels: HashMap<Vector2, isize>,
    map: HashMap<Vector2, Tile>,
}

impl Cave {
    fn new(target: Vector2, depth: isize) -> Self {
        Cave {
            depth,
            target,
            erosion_levels: HashMap::new(),
            map: HashMap::new(),
        }
    }

    fn get_erosion_level(&mut self, point: &Vector2) -> isize {
        if let Some(erosion_level) = self.erosion_levels.get(point) {
            *erosion_level
        } else {
            let geological_index = self.get_geological_index(point);
            let erosion_level = (geological_index + self.depth) % 20183;

            self.erosion_levels.insert(*point, erosion_level);
            erosion_level
        }
    }

    fn get_geological_index(&mut self, point: &Vector2) -> isize {
        if *point == Vector2(0, 0) {
            0
        } else if *point == self.target {
            0
        } else if point.1 == 0 {
            point.0 * 16807
        } else if point.0 == 0 {
            point.1 * 48271
        } else {
            self.get_erosion_level(&point.add(&LEFT)) * self.get_erosion_level(&point.add(&UP))
        }
    }

    fn get_risk_level(&mut self, point: &Vector2) -> isize {
        self.get_erosion_level(point) % 3
    }

    fn get_tile(&mut self, point: &Vector2) -> Option<Tile> {
        if point.0 < 0 || point.1 < 0 {
            None
        } else {
            match self.map.get(point) {
                Some(tile) => Some(tile.clone()),
                None => {
                    let risk_level = self.get_risk_level(point);
                    let tile = Tile::from_risk_level(risk_level);
                    self.map.insert(*point, tile);

                    Some(tile)
                }
            }
        }
    }

    fn get_neighbors(&mut self, point: &Vector2) -> Vec<(Vector2, Tile)> {
        let up_point = point.add(&UP);
        let down_point = point.add(&DOWN);
        let left_point = point.add(&LEFT);
        let right_point = point.add(&RIGHT);

        vec![up_point, down_point, left_point, right_point]
            .into_iter()
            .filter_map(|direction| match self.get_tile(&direction) {
                Some(tile) => Some((direction, tile)),
                None => None,
            })
            .collect()
    }
}

fn find_path(cave: &mut Cave) {
    let start_point = Vector2(0, 0);
    let mut distances: HashMap<Vector2, HashMap<Gear, Rc<usize>>> = HashMap::new();
    let mut previous: HashMap<Vector2, Option<Vector2>> = HashMap::new();

    distances.insert(start_point, Rc::new(0));

    let mut visit = BTreeSet::from([Node(
        start_point,
        Rc::clone(distances.get(&start_point).unwrap()),
        TORCH_GEAR,
    )]);

    while let Some(visit_node) = visit.pop_first() {
        if visit_node.0 == cave.target {
            todo!("Goal reached, at distance: {}", visit_node.1)
        }

        let neighbors = cave.get_neighbors(&visit_node.0);
        for (neighbor, neighbor_tile) in neighbors {
            let neighbor_distance = distances.entry(neighbor).or_insert(Rc::new(usize::MAX));

            match neighbor_tile {
                Tile::Rock => {
                    if visit_node.2 == NEITHER_GEAR {
                        let new_distance = *visit_node.1 + 1;
                        if new_distance < **neighbor_distance {
                            *Rc::make_mut(neighbor_distance) = new_distance;

                            let node = Node(
                                neighbor,
                                Rc::clone(distances.get(&neighbor).unwrap()),
                                visit_node.2,
                            );
                            visit.insert(node);
                        }
                    } else {
                        let new_distance = *visit_node.1 + 7;
                        if new_distance < **neighbor_distance {
                            *Rc::make_mut(neighbor_distance) = new_distance;
                            let a_node = Node(
                                neighbor.clone(),
                                Rc::clone(distances.get(&neighbor).unwrap()),
                                TORCH_GEAR,
                            );
                            let b_node = Node(
                                neighbor,
                                Rc::clone(distances.get(&neighbor).unwrap()),
                                CLIMBING_GEAR,
                            );
                            visit.insert(a_node);
                            visit.insert(b_node);
                        }
                    }
                }
                Tile::Wet => {
                    if visit_node.2 == TORCH_GEAR {
                        let new_distance = *visit_node.1 + 1;
                        if new_distance < **neighbor_distance {
                            *Rc::make_mut(neighbor_distance) = new_distance;

                            let node = Node(
                                neighbor,
                                Rc::clone(distances.get(&neighbor).unwrap()),
                                visit_node.2,
                            );
                            visit.insert(node);
                        }
                    } else {
                        let new_distance = *visit_node.1 + 7;

                        if new_distance < **neighbor_distance {
                            *Rc::make_mut(neighbor_distance) = new_distance;
                            let a_node = Node(
                                neighbor.clone(),
                                Rc::clone(distances.get(&neighbor).unwrap()),
                                CLIMBING_GEAR,
                            );
                            let b_node = Node(
                                neighbor,
                                Rc::clone(distances.get(&neighbor).unwrap()),
                                NEITHER_GEAR,
                            );
                            visit.insert(a_node);
                            visit.insert(b_node);
                        }
                    }
                }
                Tile::Narrow => {
                    if visit_node.2 == CLIMBING_GEAR {
                        let new_distance = *visit_node.1 + 1;

                        if new_distance < **neighbor_distance {
                            *Rc::make_mut(neighbor_distance) = new_distance;

                            let node = Node(
                                neighbor,
                                Rc::clone(distances.get(&neighbor).unwrap()),
                                visit_node.2,
                            );
                            visit.insert(node);
                        }
                    } else {
                        let new_distance = *visit_node.1 + 7;

                        if new_distance < **neighbor_distance {
                            *Rc::make_mut(neighbor_distance) = new_distance;
                            let a_node = Node(
                                neighbor.clone(),
                                Rc::clone(distances.get(&neighbor).unwrap()),
                                TORCH_GEAR,
                            );
                            let b_node = Node(
                                neighbor,
                                Rc::clone(distances.get(&neighbor).unwrap()),
                                NEITHER_GEAR,
                            );
                            visit.insert(a_node);
                            visit.insert(b_node);
                        }
                    }
                }
            }
            *previous.entry(neighbor).or_insert(None) = Some(visit_node.0.clone());
        }
    }
}

fn read_input() -> Cave {
    let mut depth_line = String::new();
    let mut target_line = String::new();

    let _ = std::io::stdin().read_line(&mut depth_line);
    let _ = std::io::stdin().read_line(&mut target_line);

    let target_vector = target_line.trim().replace("target: ", "");
    let (x_str, y_str) = target_vector.split_once(",").unwrap();

    Cave::new(
        Vector2(x_str.parse().unwrap(), y_str.parse().unwrap()),
        depth_line.trim().replace("depth: ", "").parse().unwrap(),
    )
}

fn part1() -> Option<isize> {
    let mut cave = read_input();
    let mut result = 0;
    for x in 0..=cave.target.0 {
        for y in 0..=cave.target.1 {
            result += cave.get_risk_level(&Vector2(x, y))
        }
    }
    Some(result)
}

fn part2() -> Option<isize> {
    let mut cave = read_input();
    find_path(&mut cave);
    None
}

fn main() {
    println!("--- Day 22: Mode Maze ---");
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
