use std::collections::HashMap;
use std::env;
use std::fs;

type Vector = (i32, i32);
type Position = (i32, i32);

#[derive(Debug, Clone, Copy)]
struct RopeEdge(Position);

impl RopeEdge {
    fn new() -> RopeEdge {
        RopeEdge((0, 0))
    }

    fn move_to(self: &mut RopeEdge, vector: Vector) {
        self.0 .0 += vector.0;
        self.0 .1 += vector.1;
    }

    fn move_closer(self: &mut RopeEdge, head: &RopeEdge) {
        let x_diff = self.0 .0 - head.0 .0;
        let y_diff = self.0 .1 - head.0 .1;

        // They are touching, let it be
        if x_diff == 0 && y_diff == 0 {
            // Do not do anything
            return;
        }

        // Same column, move up or down
        if self.0 .0 == head.0 .0 {
            if y_diff < -1 {
                self.0 .1 += 1;
                return;
            } else if y_diff > 1 {
                self.0 .1 -= 1;
                return;
            }
        }

        // Same row, move right or left
        if self.0 .1 == head.0 .1 {
            if x_diff < -1 {
                self.0 .0 += 1;
                return;
            } else if x_diff > 1 {
                self.0 .0 -= 1;
            }
        }

        // Move diagonally
        if x_diff.abs() > 1 && y_diff.abs() > 0 || x_diff.abs() > 0 && y_diff.abs() > 1 {
            self.0 .0 += x_diff.clamp(-1, 1) * -1;
            self.0 .1 += y_diff.clamp(-1, 1) * -1;
            return;
        }

        // Too close just return
    }
}

#[derive(Debug)]
struct Direction(Vector, i32);

impl Direction {
    fn from_str(str: &str) -> Direction {
        let mut splitted = str.split(' ');
        let dir_str = splitted.next().unwrap();
        let dir_count = splitted.next().unwrap().parse::<i32>().unwrap();

        if dir_str == "U" {
            Direction((0, -1), dir_count)
        } else if dir_str == "R" {
            Direction((1, 0), dir_count)
        } else if dir_str == "D" {
            Direction((0, 1), dir_count)
        } else {
            Direction((-1, 0), dir_count)
        }
    }
}

fn parse_input() -> String {
    fs::read_to_string("input.txt").expect("Unable to read file!")
}

fn parse_directions() -> Vec<Direction> {
    parse_input()
        .lines()
        .map(|line| Direction::from_str(line))
        .collect()
}

fn part1() {
    let directions = parse_directions();
    let mut map: HashMap<Position, bool> = Default::default();

    let head = &mut RopeEdge::new();
    let tail = &mut RopeEdge::new();

    map.insert(tail.0, true);

    for direction in directions {
        for _i in 0..direction.1 {
            head.move_to(direction.0);
            tail.move_closer(head);

            map.insert(tail.0, true);
        }
    }
    println!("Result: {:?}", map.len());
}

fn part2() {
    let directions = parse_directions();
    let mut map: HashMap<Position, bool> = Default::default();
    let parts = 10;

    let head = &mut RopeEdge::new();
    let mut body_parts = vec![*head];
    for _i in 0..(parts - 2) {
        body_parts.push(RopeEdge::new())
    }
    let tail = &mut RopeEdge::new();
    body_parts.push(*tail);

    map.insert(tail.0, true);

    for direction in directions {
        for _i in 0..direction.1 {
            for part_index in 0..body_parts.len() {
                if part_index == 0 {
                    (&mut body_parts[part_index]).move_to(direction.0);
                } else {
                    let (left, right) = body_parts.split_at_mut(part_index);
                    (&mut right[0]).move_closer(&left[part_index - 1]);

                    if part_index == (parts - 1) {
                        map.insert((&mut right[0]).0, true);
                    }
                }
            }
        }
    }
    println!("Result: {:?}", map.len());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let part = &args[1];

    if part == "1" {
        part1();
    } else if part == "2" {
        part2();
    }
}
