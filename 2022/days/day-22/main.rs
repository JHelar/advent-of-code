use std::env;
use std::fmt::Display;
use std::fs;

type CubeSide = Vec<Vec<Tile>>;

#[derive(Debug, Clone, Copy)]
enum Tile {
    Void,
    Path,
    Wall,
    Visited(usize)
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Tile::Path => ".",
            Tile::Wall => "#",
            Tile::Void => " ",
            Tile::Visited(facing) => {
                match facing {
                    0 => ">",
                    1 => "v",
                    2 => "<",
                    3 => "^",
                    _ => todo!("Invalid direction {}", facing),
                }
            }
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
        Self::get_vector(self.facing)
    }

    fn get_vector(facing: usize) -> (i32, i32) {
        match facing {
            0 => (1, 0),
            1 => (0, 1),
            2 => (-1, 0),
            3 => (0, -1),
            _ => todo!("Invalid direction {}", facing),
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
    cube: Vec<CubeSide>,
    current_side: usize,
    cube_size: usize,
    monkey: Monkey,
}

impl Map {
    fn new() -> Self {
        Self {
            jungle: Vec::default(),
            monkey: Monkey::new(),
            jungle_width: 0,
            cube: Vec::new(),
            current_side: 0,
            cube_size: 0,
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

    fn initialize_as_cube(&mut self, cube_size: usize) {
        let mut cube: Vec<CubeSide> = Vec::new();
        self.cube_size = cube_size;

        for y_chunk in self.jungle.chunks(cube_size) {
            let mut sides = Vec::new();

            for row in y_chunk {
                let filtered_row = row
                    .iter()
                    .filter(|tile| !matches!(tile, Tile::Void))
                    .map(|tile| *tile)
                    .collect::<Vec<Tile>>();

                let row_itor = filtered_row.chunks(cube_size).into_iter().enumerate();

                for (index, side_row) in row_itor {
                    let row = side_row.into_iter().map(|tile| *tile).collect();

                    if sides.len() < (index + 1) {
                        sides.push(vec![row]);
                    } else {
                        sides.get_mut(index).unwrap().push(row);
                    }
                }
            }

            cube.append(&mut sides);
        }

        self.cube = cube;

        // Rotate
        // self.rotate_face_clockwise(0, 0); // front_face
        // self.rotate_face_clockwise(4, 0); // back_face
        // self.rotate_face_clockwise(5, 3); // top_face
        // self.rotate_face_clockwise(1, 0); // right_face
        // self.rotate_face_clockwise(2, 0); // bottom_face
        // self.rotate_face_clockwise(3, 2); // left_face
    }

    fn get_column(&mut self, x: i32) -> Vec<Tile> {
        let mut column = Vec::new();
        for row in self.jungle.iter() {
            column.push(row[x as usize].clone());
        }

        column
    }

    /**
     * 0 => write!(f, ">"),
            1 => write!(f, "v"),
            2 => write!(f, "<"),
            3 => write!(f, "^"),
     */

    fn get_faces(
        &self,
        (x, y): (i32, i32),
    ) -> (
        (usize, usize, (i32, i32)),
        (usize, usize, (i32, i32)),
        (usize, usize, (i32, i32)),
        (usize, usize, (i32, i32)),
    ) {
        let max_size = self.cube_size as i32 - 1;
        match self.current_side {
            0 => (
                (5, 0, (0, x)),
                (1, 0, (0, y)),
                (2, 1, (x, 0)),
                (3, 0, (0, max_size - y)),
            ),
            1 => (
                (5, 3, (x, max_size)),
                (4, 2, (max_size, max_size - y)),
                (2, 2, (max_size, x)),
                (0, 2, (max_size, y)),
            ),
            2 => (
                (0, 3, (x, max_size)),
                (1, 3, (y, max_size)),
                (4, 1, (x, 0)),
                (3, 1, (y, 0)),
            ),
            3 => (
                (2, 0, (0, x)),
                (4, 0, (0, y)),
                (5, 1, (x, 0)),
                (0, 0, (0, max_size - y)),
            ),
            4 => (
                (2, 3, (x, max_size)),
                (1, 2, (max_size, max_size - y)),
                (5, 2, (max_size, x)),
                (3, 2, (max_size, y)),
            ),
            5 => (
                (3, 3, (x, max_size)),
                (4, 3, (y, max_size)),
                (1, 1, (x, 0)),
                (0, 1, (y, 0)),
            ),
            _ => todo!("Invalid face!"),
        }
    }

    fn move_monkey_jungle(&mut self, amount: i32) {
        let (delta_x, delta_y) = self.monkey.vector();

        for _i in 0..amount {
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
                Tile::Visited(_) => {
                    self.monkey.position.0 = new_x;
                    self.monkey.position.1 = new_y;
                }
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

    fn move_monkey_cube(&mut self, amount: i32) {
        for _i in 0..amount {
            let mut new_face = self.current_side;
            let mut new_facing = self.monkey.facing;

            let (delta_x, delta_y) = Monkey::get_vector(new_facing);
            let mut new_y = self.monkey.position.1 + delta_y;
            let mut new_x = self.monkey.position.0 + delta_x;

            if new_x < 0 {
                (new_face, new_facing, (new_x, new_y)) = self.get_faces((new_x, new_y)).3;
            } else if new_x >= self.cube_size as i32 {
                (new_face, new_facing, (new_x, new_y)) = self.get_faces((new_x, new_y)).1;
            } else if new_y < 0 {
                (new_face, new_facing, (new_x, new_y)) = self.get_faces((new_x, new_y)).0;
            } else if new_y >= self.cube_size as i32 {
                (new_face, new_facing, (new_x, new_y)) = self.get_faces((new_x, new_y)).2;
            }

            let tile = &self.cube[new_face as usize][new_y as usize][new_x as usize];
            match tile {
                Tile::Visited(_) => {
                    self.current_side = new_face;
                    self.monkey.facing = new_facing;
                    self.monkey.position.0 = new_x;
                    self.monkey.position.1 = new_y;

                    self.cube[new_face as usize][new_y as usize][new_x as usize] = Tile::Visited(self.monkey.facing);
                },
                Tile::Path => {
                    self.current_side = new_face;
                    self.monkey.facing = new_facing;
                    self.monkey.position.0 = new_x;
                    self.monkey.position.1 = new_y;

                    self.cube[new_face as usize][new_y as usize][new_x as usize] = Tile::Visited(self.monkey.facing);
                }
                Tile::Wall => {
                    break;
                }
                Tile::Void => {}
            }
        }
    }

    fn move_monkey(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Forward(amount) => {
                if self.cube.is_empty() {
                    self.move_monkey_jungle(*amount);
                } else {
                    self.move_monkey_cube(*amount);
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
        if !self.cube.is_empty() {
            for (index, side) in self.cube.iter().enumerate() {
                if index == self.current_side {
                    writeln!(f, "Side: {}", index).unwrap();
                    for (y, row) in side.iter().enumerate() {
                        for (x, tile) in row.iter().enumerate() {
                            if index == self.current_side
                                && x as i32 == self.monkey.position.0
                                && y as i32 == self.monkey.position.1
                            {
                                write!(f, "{}", self.monkey).unwrap();
                            } else {
                                write!(f, "{tile}").unwrap();
                            }
                        }
                        write!(f, "\n").unwrap();
                    }
                }
            }
        } else {
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

fn part2() {
    let (instructions, mut map) = parse_input();
    map.initialize_as_cube(50);

    for instruction in instructions.iter() {
        map.move_monkey(instruction);
    }

    let side_offsets = vec![
        (50, 0),
        (100, 0),
        (50, 50),
        (0, 100),
        (50, 100),
        (0, 150),
    ];

    let row = (map.monkey.position.1 + 1) + side_offsets[map.current_side].1;
    let column = (map.monkey.position.0 + 1) + side_offsets[map.current_side].0;
    let facing = map.monkey.facing;

    let result = row * 1000 + 4 * column + facing as i32;

    println!("Result: {result}");
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
