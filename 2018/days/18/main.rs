use std::{f32::MIN, fmt::Display};
use hashbrown::HashMap;

use vector2::Vector2;

mod vector2;

const DIRECTIONS: [Vector2; 8] = [
    Vector2(0, -1),
    Vector2(0, 1),
    Vector2(-1, 0),
    Vector2(1, 0),
    Vector2(-1, -1),
    Vector2(1, -1),
    Vector2(-1, 1),
    Vector2(1, 1),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Acre {
    Open,
    Trees,
    Lumberyard,
}

type Area = Vec<Vec<Acre>>;

impl Acre {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Open,
            '|' => Self::Trees,
            '#' => Self::Lumberyard,
            _ => panic!("Unknown Acre type '{c}'"),
        }
    }
}

impl Display for Acre {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Acre::Open => write!(f, "."),
            Acre::Trees => write!(f, "|"),
            Acre::Lumberyard => write!(f, "#"),
        }
    }
}

fn read_input() -> Area {
    std::io::stdin()
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| line.trim().chars().map(Acre::from_char).collect())
        .collect()
}

fn print_area(area: &Area) {
    print!("\n");
    for row in area {
        for acre in row {
            print!("{acre}");
        }
        print!("\n");
    }
}

fn get_adjacent(point: Vector2, area: &Area) -> Vec<Acre> {
    DIRECTIONS
        .iter()
        .filter_map(|direction| {
            let dir_point = point.add(direction);
            get_area_acre(dir_point, area).cloned()
        })
        .collect()
}

fn get_area_acre(point: Vector2, area: &Area) -> Option<&Acre> {
    if let Some(row) = area.get(point.1 as usize) {
        row.get(point.0 as usize)
    } else {
        None
    }
}

fn get_area_acre_mut(point: Vector2, area: &mut Area) -> Option<&mut Acre> {
    if let Some(row) = area.get_mut(point.1 as usize) {
        row.get_mut(point.0 as usize)
    } else {
        None
    }
}

fn step(area: &Area) -> Area {
    let mut new_area = area.clone();

    for x in 0..(area.len() as isize) {
        for y in 0..(area.len() as isize) {
            let point = Vector2(x, y);
            let adjacent = get_adjacent(point, area);

            match get_area_acre_mut(point, &mut new_area) {
                Some(acre) => match acre {
                    Acre::Open => {
                        let count = adjacent.iter().fold(0, |acc, adjacent_acre| {
                            if matches!(adjacent_acre, Acre::Trees) {
                                acc + 1
                            } else {
                                acc
                            }
                        });

                        if count >= 3 {
                            *acre = Acre::Trees;
                        }
                    }
                    Acre::Trees => {
                        let count = adjacent.iter().fold(0, |acc, adjacent_acre| {
                            if matches!(adjacent_acre, Acre::Lumberyard) {
                                acc + 1
                            } else {
                                acc
                            }
                        });

                        if count >= 3 {
                            *acre = Acre::Lumberyard;
                        }
                    }
                    Acre::Lumberyard => {
                        if !adjacent.contains(&Acre::Lumberyard) || !adjacent.contains(&Acre::Trees)
                        {
                            *acre = Acre::Open
                        }
                    }
                },
                None => panic!("No tile"),
            }
        }
    }
    new_area
}

fn calculate_score(area: &Area) -> usize {
    let mut trees = 0;
    let mut lumberyards = 0;

    for row in area {
        for acre in row {
            match acre {
                Acre::Lumberyard => lumberyards += 1,
                Acre::Trees => trees += 1,
                Acre::Open => {}
            }
        }
    }
    trees * lumberyards
}

fn part1() -> Option<usize> {
    let mut area = read_input();
    const MINUTES: usize = 10;
    for _ in 1..=MINUTES {
        area = step(&mut area);
    }
    
    print_area(&area);
    Some(calculate_score(&area))
}

fn part2() -> Option<usize> {
  let mut area = read_input();
  let mut mem: HashMap<Area, usize> = HashMap::new();
  const MINUTES: usize = 1_000_000_000;

  for minute in 1..=MINUTES {
      area = step(&mut area);
      if let Some(previous_minute) = mem.get(&area) {
        let diff = minute - previous_minute;
        if (MINUTES - previous_minute) % diff == 0 {
          break;
        }
      } else {
        mem.insert(area.clone(), minute);
      }
  }
  
  print_area(&area);
  Some(calculate_score(&area))
}

fn main() {
    println!("--- Day 18: Settlers of The North Pole ---");
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
