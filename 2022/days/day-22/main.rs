#![feature(iter_next_chunk)]

use std::env;
use std::fmt::Display;
use std::fs;

type CubeSide = Vec<Vec<Tile>>;

#[derive(Debug, Clone, Copy)]
enum Tile {
    Void,
    Path,
    Wall,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Tile::Path => ".",
            Tile::Wall => "#",
            Tile::Void => " ",
        };
        write!(f, "{value}")
    }
}

#[derive(Debug)]
enum Instruction {
    Forward(i32),
    Clockwise,
    Counterclockwise,
}

impl Instruction {
    fn from_line(line: &str) -> Vec<Instruction> {
        let mut peekable = line.chars().peekable();
        let mut instructions: Vec<Instruction> = Vec::default();

        while let Some(token) = peekable.next() {
            let instruction = match token {
                'R' => Instruction::Clockwise,
                'L' => Instruction::Counterclockwise,
                _ => {
                    let mut number_vec = vec![token.to_string()];
                    while let Some(next_token) = peekable.peek() {
                        if !next_token.is_numeric() {
                            break;
                        }
                        number_vec.push(peekable.next().unwrap().to_string());
                    }

                    let number = number_vec.join("").parse::<i32>().unwrap();
                    Instruction::Forward(number)
                }
            };
            instructions.push(instruction);
        }
        instructions
    }
}

#[derive(Debug)]
struct Monkey {
    facing: usize,
    position: (i32, i32),
}

impl Monkey {
    fn new() -> Self {
        Monkey {
            facing: 0,
            position: (0, 0),
        }
    }

    fn vector(&self) -> (i32, i32) {
        match self.facing {
            0 => (1, 0),
            1 => (0, 1),
            2 => (-1, 0),
            3 => (0, -1),
            _ => todo!("Invalid direction {}", self.facing),
        }
    }
}

impl Display for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.facing {
            0 => write!(f, ">"),
            1 => write!(f, "v"),
            2 => write!(f, "<"),
            3 => write!(f, "^"),
            _ => todo!("Invalid direction {}", self.facing),
        }
    }
}

#[derive(Debug)]
struct Map {
    jungle: Vec<Vec<Tile>>,
    jungle_width: usize,
    cube: Vec<Vec<CubeSide>>,
    monkey: Monkey,
}

impl Map {
    fn new() -> Self {
        Self {
            jungle: Vec::default(),
            monkey: Monkey::new(),
            jungle_width: 0,
            cube: Vec::new(),
        }
    }

    fn push_row(&mut self, row: Vec<Tile>) {
        let row_length = row.len();

        self.jungle.push(row);
        self.jungle_width = self.jungle_width.max(row_length);
    }

    fn initialize_as_jungle(&mut self) {
        for row in self.jungle.iter_mut() {
            let diff = self.jungle_width - row.len();
            if diff > 0 {
                (0..diff).for_each(|_| {
                    row.push(Tile::Void);
                })
            }
        }
        for (x, tile) in self.jungle[0].iter().enumerate() {
            if matches!(tile, Tile::Path) {
                self.monkey.position.0 = x as i32;
                self.monkey.position.1 = 0;
                break;
            }
        }
    }

    fn initialize_as_cube(&mut self) {
        let mut cube: Vec<Vec<CubeSide>> = Vec::new();

        for y_chunk in self.jungle.chunks(50) {
            let mut side_1: CubeSide = Vec::new();
            let mut side_2: CubeSide = Vec::new();

            for row in y_chunk {
                let mut filtered_row = row
                    .iter()
                    .filter(|tile| !matches!(tile, Tile::Void))
                    .map(|tile| *tile)
                    .collect::<Vec<Tile>>();

                let mut row_itor = filtered_row.chunks(50).into_iter();

                let first = row_itor
                    .next()
                    .unwrap()
                    .into_iter()
                    .map(|tile| *tile)
                    .collect();

                side_1.push(first);

                if let Some(second) = row_itor.next() {
                    side_2.push(second.into_iter().map(|tile| *tile).collect());
                }
            }

            let mut cube_row = vec![side_1];

            if !side_2.is_empty() {
                cube_row.push(side_2);
            }

            cube.push(cube_row);
        }

        self.cube = cube;
    }

    fn get_column(&mut self, x: i32) -> Vec<Tile> {
        let mut column = Vec::new();
        for row in self.jungle.iter() {
            column.push(row[x as usize].clone());
        }

        column
    }

    fn move_monkey(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Forward(amount) => {
                let (delta_x, delta_y) = self.monkey.vector();

                for _i in 0..*amount {
                    let mut new_y = (self.monkey.position.1 + delta_y) % self.jungle.len() as i32;
                    let mut new_x = (self.monkey.position.0 + delta_x) % self.jungle_width as i32;

                    if new_y < 0 {
                        new_y = (self.jungle.len() as i32) - 1;
                    }

                    if new_x < 0 {
                        new_x = (self.jungle_width as i32) - 1;
                    }

                    let tile = &self.jungle[new_y as usize][new_x as usize];
                    match tile {
                        Tile::Path => {
                            self.monkey.position.0 = new_x;
                            self.monkey.position.1 = new_y;
                        }
                        Tile::Wall => {}
                        Tile::Void => {
                            // Fell off!
                            match self.monkey.facing {
                                0 => {
                                    let row = &self.jungle[new_y as usize];
                                    // Find left most tile that is not void
                                    for (row_x, tile) in row.iter().enumerate() {
                                        if matches!(tile, Tile::Void) {
                                            continue;
                                        }
                                        if matches!(tile, Tile::Path) {
                                            self.monkey.position.0 = row_x as i32;
                                            self.monkey.position.1 = new_y;
                                            break;
                                        } else {
                                            // Hit rock just break without position update
                                            break;
                                        }
                                    }
                                }
                                2 => {
                                    let row = &self.jungle[new_y as usize];
                                    // Find right most tile that is not void
                                    for (row_x, tile) in row.iter().enumerate().rev() {
                                        if matches!(tile, Tile::Void) {
                                            continue;
                                        }
                                        if matches!(tile, Tile::Path) {
                                            self.monkey.position.0 = row_x as i32;
                                            self.monkey.position.1 = new_y;
                                            break;
                                        } else {
                                            // Hit rock just break without position update
                                            break;
                                        }
                                    }
                                }
                                1 => {
                                    let column = self.get_column(new_x);
                                    // Find top most tile that is not void
                                    for (column_y, tile) in column.iter().enumerate() {
                                        if matches!(tile, Tile::Void) {
                                            continue;
                                        }
                                        if matches!(tile, Tile::Path) {
                                            self.monkey.position.0 = new_x;
                                            self.monkey.position.1 = column_y as i32;
                                            break;
                                        } else {
                                            // Hit rock just break without position update
                                            break;
                                        }
                                    }
                                }
                                3 => {
                                    let column = self.get_column(new_x);
                                    // Find bottom most tile that is not void
                                    for (column_y, tile) in column.iter().enumerate().rev() {
                                        if matches!(tile, Tile::Void) {
                                            continue;
                                        }
                                        if matches!(tile, Tile::Path) {
                                            self.monkey.position.0 = new_x;
                                            self.monkey.position.1 = column_y as i32;
                                            break;
                                        } else {
                                            // Hit rock just break without position update
                                            break;
                                        }
                                    }
                                }
                                _ => todo!("Invalid direction"),
                            }
                        }
                    }
                }
            }
            Instruction::Clockwise => {
                self.monkey.facing = (self.monkey.facing + 1) % 4;
            }
            Instruction::Counterclockwise => {
                if self.monkey.facing == 0 {
                    self.monkey.facing = 3;
                } else {
                    self.monkey.facing = self.monkey.facing - 1;
                }
            }
        }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (y, row) in self.jungle.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if x as i32 == self.monkey.position.0 && y as i32 == self.monkey.position.1 {
                    write!(f, "{}", self.monkey).unwrap();
                } else {
                    write!(f, "{tile}").unwrap();
                }
            }
            write!(f, "\n").unwrap();
        }

        Ok(())
    }
}

fn parse_input() -> (Vec<Instruction>, Map) {
    let input = fs::read_to_string("input.txt").expect("Unable to read file!");

    let lines = input.lines().collect::<Vec<&str>>();

    let instructions_str = lines.iter().last().unwrap();
    let instructions = Instruction::from_line(&instructions_str);

    let mut map = Map::new();

    lines.iter().take(lines.len() - 2).for_each(|line| {
        let mut row = Vec::new();
        line.chars().for_each(|s| match s {
            '.' => row.push(Tile::Path),
            '#' => row.push(Tile::Wall),
            ' ' => row.push(Tile::Void),
            _ => {}
        });

        map.push_row(row);
    });

    (instructions, map)
}

fn part1() {
    let (instructions, mut map) = parse_input();
    map.initialize_as_jungle();

    for instruction in instructions.iter() {
        map.move_monkey(instruction);
    }

    let column = map.monkey.position.0 + 1;
    let row = map.monkey.position.1 + 1;
    let facing = map.monkey.facing;

    let result = row * 1000 + 4 * column + facing as i32;

    println!("Result: {result}");
}

fn part2() {}

fn main() {
    let args: Vec<String> = env::args().collect();
    let part = &args[1];

    if part == "1" {
        part1();
    } else if part == "2" {
        part2();
    }
}
