type Point = (isize, isize);
type Puzzle = Vec<Vec<char>>;

const XMAS_WORD_DIRECTIONS: [[Point; 4]; 8] = [
    [(0, 0), (1, 0), (2, 0), (3, 0)],
    [(0, 0), (-1, 0), (-2, 0), (-3, 0)],
    [(0, 0), (0, -1), (0, -2), (0, -3)],
    [(0, 0), (0, 1), (0, 2), (0, 3)],
    [(0, 0), (1, -1), (2, -2), (3, -3)],
    [(0, 0), (-1, -1), (-2, -2), (-3, -3)],
    [(0, 0), (1, 1), (2, 2), (3, 3)],
    [(0, 0), (-1, 1), (-2, 2), (-3, 3)],
];

const X_MAS_WORD_DIRECTIONS: [[Point; 3]; 2] =
    [[(1, -1), (0, 0), (-1, 1)], [(-1, -1), (0, 0), (1, 1)]];

const RIGHT: Point = (1, 0);

fn read_input() -> Puzzle {
    std::io::stdin()
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| line.trim().chars().collect())
        .collect()
}

fn add(point: Point, other: Point) -> Point {
    (point.0 + other.0, point.1 + other.1)
}

fn get_puzzle_char(point: Point, puzzle: &Puzzle) -> Option<&char> {
    if point.1 < 0 || point.0 < 0 {
        None
    } else {
        match puzzle.get(point.1 as usize) {
            Some(row) => row.get(point.0 as usize),
            None => None,
        }
    }
}

fn get_xmas_word(point: Point, direction: [Point; 4], puzzle: &Puzzle) -> String {
    direction
        .iter()
        .filter_map(|delta| {
            let point = add(point, *delta);
            get_puzzle_char(point, puzzle)
        })
        .collect()
}

fn get_x_mas_word(point: Point, direction: [Point; 3], puzzle: &Puzzle) -> String {
    direction
        .iter()
        .filter_map(|delta| {
            let point = add(point, *delta);
            get_puzzle_char(point, puzzle)
        })
        .collect()
}

fn part1() -> Option<u32> {
    let puzzle = read_input();
    let puzzle_size = puzzle.len() as isize;

    let mut point: Point = (0, 0);
    let mut result = 0;

    while let Some(c) = get_puzzle_char(point, &puzzle) {
        match c {
            'X' => {
                for word_direction in XMAS_WORD_DIRECTIONS {
                    let word = get_xmas_word(point, word_direction, &puzzle);
                    if word == "XMAS" {
                        result += 1;
                    }
                }
            }
            _ => {}
        }

        point = add(point, RIGHT);
        if point.0 >= puzzle_size {
            point.0 = 0;
            point.1 += 1;
        }
    }
    Some(result)
}

fn part2() -> Option<u32> {
    let puzzle = read_input();
    let puzzle_size = puzzle.len() as isize;

    let mut point: Point = (0, 0);
    let mut result = 0;

    while let Some(c) = get_puzzle_char(point, &puzzle) {
        match c {
            'A' => {
                let is_x_mas = X_MAS_WORD_DIRECTIONS.iter().all(|word_direction| {
                    let word = get_x_mas_word(point, *word_direction, &puzzle);
                    word == "MAS" || word == "SAM"
                });
                if is_x_mas {
                    result += 1;
                }
            }
            _ => {}
        }

        point = add(point, RIGHT);
        if point.0 >= puzzle_size {
            point.0 = 0;
            point.1 += 1;
        }
    }
    Some(result)
}

fn main() {
    println!("--- Day 4: Ceres Search ---");
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
