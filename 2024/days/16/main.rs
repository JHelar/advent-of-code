use hashbrown::{HashMap, HashSet};
use ratatui::{layout::Rect, style::Stylize, widgets::Paragraph, Frame};
use std::{fmt::Display, thread::sleep, time::Duration};
use vector2::{Vector2, DOWN, LEFT, RIGHT, UP, ZERO};

mod vector2;

type Node = (Vector2, Vector2);

const NORTH: Vector2 = UP;
const SOUTH: Vector2 = DOWN;
const EAST: Vector2 = RIGHT;
const WEST: Vector2 = LEFT;

enum Tile {
    Air(Vector2),
    Visited(Vector2, Vector2),
    Reindeer(Vector2),
    Start(Vector2),
    End(Vector2),
    Wall(Vector2),
}

impl Tile {
    fn get_position(&self) -> &Vector2 {
        match self {
            Self::Air(position) => position,
            Self::Visited(position, _) => position,
            Self::Reindeer(position) => position,
            Self::Start(position) => position,
            Self::End(position) => position,
            Self::Wall(position) => position,
        }
    }
}

struct Maze {
    map: Vec<Tile>,
    size: Vector2,
}

impl Maze {
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

    fn draw(&self, frame: &mut Frame) {
        for y in 0..self.size.1 {
            for x in 0..self.size.0 {
                let paragraph = match self.get_tile(&Vector2(x, y)) {
                    Some(tile) => match tile {
                        Tile::Visited(_, direction) => match *direction {
                            NORTH => Paragraph::new("^").white(),
                            SOUTH => Paragraph::new("v").white(),
                            EAST => Paragraph::new(">").white(),
                            WEST => Paragraph::new("<").white(),
                            ZERO => Paragraph::new("O").white(),
                            _ => panic!("Bad visited"),
                        },
                        Tile::Reindeer(_) => Paragraph::new("@").red().bold(),
                        Tile::Air(_) => Paragraph::new(" ").gray(),
                        Tile::Start(_) => Paragraph::new("S").gray(),
                        Tile::End(_) => Paragraph::new("E").gray(),
                        Tile::Wall(_) => Paragraph::new("#").green(),
                    },
                    None => panic!("ooops"),
                };
                let rect = Rect::new(x as u16, y as u16, 1, 1);
                if rect.x >= frame.area().width || rect.y >= frame.area().height {
                  continue;
                }
                frame.render_widget(paragraph, rect);
            }
        }
    }
}

impl Display for Maze {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.size.1 {
            for x in 0..self.size.0 {
                match self.get_tile(&Vector2(x, y)) {
                    Some(tile) => match tile {
                        Tile::Visited(_, direction) => match *direction {
                            NORTH => write!(f, "^"),
                            SOUTH => write!(f, "v"),
                            EAST => write!(f, ">"),
                            WEST => write!(f, "<"),
                            ZERO => write!(f, "O"),
                            _ => panic!("Bad visited"),
                        },
                        Tile::Reindeer(_) => write!(f, "@"),
                        Tile::Air(_) => write!(f, "."),
                        Tile::Start(_) => write!(f, "S"),
                        Tile::End(_) => write!(f, "E"),
                        Tile::Wall(_) => write!(f, "#"),
                    },
                    None => panic!("ooops"),
                }?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn read_input() -> Maze {
    let mut tiles = Vec::new();
    let mut size = ZERO;

    for (y, row) in std::io::stdin()
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| line.trim().to_string())
        .enumerate()
    {
        size.1 = row.len() as isize;

        for (x, tile_char) in row.chars().enumerate() {
            size.0 = size.0.max(x as isize + 1);
            let position = Vector2(x as isize, y as isize);
            let tile = match tile_char {
                '#' => Tile::Wall(position),
                '.' => Tile::Air(position),
                'E' => Tile::End(position),
                'S' => Tile::Start(position),
                _ => panic!("Unknown tile: {tile_char}"),
            };
            tiles.push(tile);
        }
    }

    Maze { map: tiles, size }
}

fn find_best_path(maze: &mut Maze) -> (usize, HashSet<Node>) {
  let mut terminal = ratatui::init();
    terminal.clear().unwrap();
    
    let start_position = maze
        .map
        .iter()
        .find(|t| matches!(t, Tile::Start(_)))
        .unwrap()
        .get_position()
        .clone();

    let end_position = maze
        .map
        .iter()
        .find(|t| matches!(t, Tile::End(_)))
        .unwrap()
        .get_position()
        .clone();

    let mut costs: HashMap<Node, usize> = HashMap::new();
    costs.insert((start_position.clone(), EAST), 0);

    let mut visit: Vec<(Vector2, Vector2, HashMap<Node, Option<Node>>)> = Vec::new();
    visit.push((
        start_position.clone(),
        EAST,
        HashMap::from([((start_position.clone(), EAST), None)]),
    ));

    let mut best_cost = usize::MAX;

    let mut path = HashSet::new();

    while let Some((current_position, direction, mut previous)) = visit.pop() {
        let current_cost = costs.get(&(current_position, direction)).unwrap().clone();

        if current_position == end_position && current_cost <= best_cost {
            best_cost = current_cost;
            let mut position = (current_position, direction);
            path.insert(position.clone());

            let mut this_path = vec![position.clone()];

            while let Some(previous_position) = previous.get(&position).unwrap_or(&None) {
                position = *previous_position;
                if path.contains(&position) {
                  continue;
                }
                this_path.push(position.clone());
            }
            this_path.reverse();

            let mut previous = this_path.first().unwrap();
            *maze.get_tile_mut(&previous.0).unwrap() = Tile::Reindeer(previous.0);

            for position in &this_path[1..] {
              path.insert(position.clone());
              terminal.draw(|frame| maze.draw(frame)).unwrap();
              sleep(Duration::from_millis(100));
              *maze.get_tile_mut(&previous.0).unwrap() = Tile::Visited(previous.0, previous.1);
              previous = position; 
              *maze.get_tile_mut(&previous.0).unwrap() = Tile::Reindeer(previous.0);
            }
            continue;
        }

        let new_position = current_position.add(&direction);
        match maze.get_tile(&new_position) {
            Some(tile) => match tile {
                Tile::Air(_) | Tile::End(_) | Tile::Start(_) => {
                    let new_cost = current_cost + 1;
                    let old_cost = *costs.get(&(new_position, direction)).unwrap_or(&usize::MAX);

                    if new_cost <= old_cost {
                        previous.insert(
                            (new_position, direction),
                            Some((current_position, direction)),
                        );
                        costs.insert((new_position, direction), new_cost);
                        visit.push((new_position, direction, previous.clone()));
                    }
                }
                _ => {}
            },
            None => panic!("Oh dear"),
        }

        for current_rotation in [direction.rot_right_90(), direction.rot_left_90()] {
            let new_cost = current_cost + 1000;
            let old_cost = *costs
                .get(&(current_position, current_rotation))
                .unwrap_or(&usize::MAX);

            if new_cost <= old_cost {
                previous.insert(
                    (current_position, current_rotation),
                    Some((current_position, direction)),
                );
                costs.insert((current_position, current_rotation), new_cost);
                visit.push((current_position, current_rotation, previous.clone()));
            }
        }

        visit.sort_by(|a, b| {
            let a_cost = costs.get(&(a.0, a.1)).unwrap();
            let b_cost = costs.get(&(b.0, b.1)).unwrap();

            b_cost.cmp(a_cost)
        });
    }

    ratatui::restore();

    (best_cost, path)
}

fn part1() -> Option<usize> {
    let mut maze = read_input();
    let (cost, _) = find_best_path(&mut maze);
    Some(cost)
}

fn part2() -> Option<usize> {
    let mut maze = read_input();
    let (_, path) = find_best_path(&mut maze);

    let mut points = HashSet::new();
    for (position, _) in path.iter() {
        points.insert(*position);
    }
    Some(points.len())
}

fn main() {
    println!("--- Day 16: Reindeer Maze ---");
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
