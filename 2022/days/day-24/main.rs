use hashbrown::HashMap;
use std::cmp::Ordering;
use std::env;
use std::fmt::Display;
use std::fs;

type Map = Vec<Vec<Tile>>;
type Position = (usize, usize);
type BlizzardPos = (Position, Direction);
type Blizzards = Vec<BlizzardPos>;

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Self::Up => '^',
            Self::Down => 'v',
            Self::Left => '<',
            Self::Right => '>',
        };

        write!(f, "{c}")
    }
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd)]
enum Tile {
    Start,
    End,
    Blizzard(Vec<Direction>),
    Ground,
    Wall,
}

impl Ord for Tile {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (_, Tile::End) => Ordering::Less,
            (Tile::End, _) => Ordering::Greater,
            (_, Tile::Start) => Ordering::Less,
            (Tile::Start, _) => Ordering::Greater,
            _ => Ordering::Equal,
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Self::Blizzard(directions) => {
                if directions.len() > 1 {
                    directions.len().to_string()
                } else {
                    directions[0].to_string()
                }
            }
            Self::Start => "S".to_string(),
            Self::End => "E".to_string(),
            Self::Ground => ".".to_string(),
            Self::Wall => "#".to_string(),
        };

        write!(f, "{c}")
    }
}

impl Tile {
    fn from_char(c: char) -> Self {
        match c {
            '#' => Self::Wall,
            '.' => Self::Ground,
            '>' => Self::Blizzard(vec![Direction::Right]),
            '<' => Self::Blizzard(vec![Direction::Left]),
            '^' => Self::Blizzard(vec![Direction::Up]),
            'v' => Self::Blizzard(vec![Direction::Down]),
            _ => panic!("Unknown tile {c}"),
        }
    }
}

fn parse_input() -> (Map, Blizzards) {
    let mut blizzards: Blizzards = Vec::default();

    let mut map: Map = fs::read_to_string("input.txt")
        .expect("Unable to read file!")
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.trim()
                .chars()
                .enumerate()
                .map(|(x, c)| {
                    let tile = Tile::from_char(c);
                    match tile {
                        Tile::Blizzard(directions) => {
                            directions.iter().for_each(|direction| {
                                blizzards.push(((x, y), *direction));
                            });
                            Tile::Ground
                        }
                        _ => tile,
                    }
                })
                .collect()
        })
        .collect();

    map.first_mut().unwrap()[1] = Tile::Start;

    let last_row = map.last_mut().unwrap();
    let row_len = last_row.len();
    last_row[row_len - 1 - 1] = Tile::End;

    (map, blizzards)
}

fn print_map(map: &Map) {
    for row in map {
        for tile in row {
            print!("{tile}");
        }
        print!("\n");
    }
}

fn get_walkable_positions(
    (p_x, p_y): Position,
    blizzards: &Blizzards,
    map: &Map,
    goal_tile: &Tile,
    goal_position: &Position,
) -> Vec<(Position, Tile)> {
    let mut positions = Vec::default();

    if p_y > 0 {
        let north = (p_x, p_y - 1);
        let north_tile = map[north.1][north.0].clone();

        if !blizzards.iter().any(|(pos, _)| *pos == north)
            && (matches!(north_tile, Tile::Ground) || north_tile == *goal_tile)
        {
            positions.push((north, north_tile));
        }
    }
    if p_y < map.len() - 1 {
        let south = (p_x, p_y + 1);
        let south_tile = map[south.1][south.0].clone();

        if !blizzards.iter().any(|(pos, _)| *pos == south)
            && (matches!(south_tile, Tile::Ground) || south_tile == *goal_tile)
        {
            positions.push((south, south_tile));
        }
    }
    if p_x > 0 {
        let east = (p_x - 1, p_y);
        let east_tile = map[east.1][east.0].clone();

        if !blizzards.iter().any(|(pos, _)| *pos == east)
            && (matches!(east_tile, Tile::Ground) || east_tile == *goal_tile)
        {
            positions.push((east, east_tile));
        }
    }
    if p_x < map[0].len() - 1 {
        let west = (p_x + 1, p_y);
        let west_tile = map[west.1][west.0].clone();

        if !blizzards.iter().any(|(pos, _)| *pos == west)
            && (matches!(west_tile, Tile::Ground) || west_tile == *goal_tile)
        {
            positions.push((west, west_tile));
        }
    }

    let wait = (p_x, p_y);
    if !blizzards.iter().any(|(pos, _)| *pos == wait) {
        positions.push((wait, Tile::Ground));
    }

    positions.sort_by(|a, b| {
        let a_distance = distance_between(a.0, *goal_position);
        let b_distance = distance_between(b.0, *goal_position);
        a_distance.cmp(&b_distance)
    });

    positions.sort_by(|(_, a_tile), (_, b_tile)| b_tile.cmp(a_tile));

    positions
}

fn distance_between(pos_a: Position, pos_b: Position) -> i32 {
    ((pos_a.0 as i32) - (pos_b.0 as i32)).abs() + ((pos_a.1 as i32) - (pos_b.1 as i32)).abs()
}

fn new_blizzard_position(((x, y), direction): BlizzardPos, max_x: usize, max_y: usize) -> Position {
    match direction {
        Direction::Down => {
            let mut new_y = y + 1;
            if new_y > max_y {
                new_y = 1;
            }
            (x, new_y)
        }
        Direction::Up => {
            let mut new_y = y - 1;
            if new_y == 0 {
                new_y = max_y;
            }
            (x, new_y)
        }
        Direction::Left => {
            let mut new_x = x - 1;
            if new_x == 0 {
                new_x = max_x;
            }
            (new_x, y)
        }
        Direction::Right => {
            let mut new_x = x + 1;
            if new_x > max_x {
                new_x = 1;
            }
            (new_x, y)
        }
    }
}

fn move_blizzards(blizzards: &mut Blizzards, map: &Map) {
    let max_y = map.len() - 2;
    let max_x = map[0].len() - 2;

    for i in 0..blizzards.len() {
        let blizzard = blizzards[i];
        blizzards[i].0 = new_blizzard_position(blizzard, max_x, max_y);
    }
}

fn simulate(
    position: Position,
    minute: usize,
    current_best: usize,
    current_cycle: usize,
    blizzards: &Blizzards,
    map: &Map,
    mem: &mut HashMap<(usize, usize, usize), (usize, usize)>,
    cycle_length: usize,
    goal_tile: &Tile,
    goal_position: &Position,
) -> (usize, usize) {
    let mem_key = (position.0, position.1, current_cycle);
    if mem.contains_key(&mem_key) {
        return *mem.get(&mem_key).unwrap();
    } else {
        mem.insert(mem_key, (current_best, current_cycle));
    }

    let mut blizzard_copy = blizzards.clone();

    move_blizzards(&mut blizzard_copy, map);

    let new_positions =
        get_walkable_positions(position, &blizzard_copy, map, goal_tile, goal_position);
    
    let mut best_value = current_best;
    let mut best_cycle = current_cycle;
    let next_cycle = (current_cycle + 1) % cycle_length;
    let next_minute = minute + 1;

    for (new_position, tile) in new_positions.iter() {
        if tile == goal_tile {
            mem.insert(mem_key, (next_minute, next_cycle));
            return (next_minute, next_cycle);
        }

        if next_minute < best_value {
            let result = simulate(
                *new_position,
                next_minute,
                best_value,
                next_cycle,
                &blizzard_copy,
                map,
                mem,
                cycle_length,
                goal_tile,
                goal_position
            );
            if best_value > result.0 {
                best_value = result.0;
                best_cycle = result.1;
            }
        }
    }

    mem.insert(mem_key, (best_value, best_cycle));
    (best_value, best_cycle)
}

fn get_cycle_length(blizzards: &Blizzards, map: &Map) -> usize {
    let mut mem: HashMap<Vec<((usize, usize), Direction)>, i32> = HashMap::default();
    let mut blizzard_copy = blizzards.clone();

    mem.insert(blizzard_copy.clone(), 0);

    move_blizzards(&mut blizzard_copy, &map);
    let mut cycle_length = 1;

    while !mem.contains_key(&blizzard_copy) {
        move_blizzards(&mut blizzard_copy, &map);
        cycle_length += 1;
    }

    cycle_length
}

fn part1() {
    let (map, blizzards) = parse_input();

    let cycle_length = get_cycle_length(&blizzards, &map);

    let mut mem: HashMap<(usize, usize, usize), (usize, usize)> = HashMap::default();
    let player_pos = (1, 0);
    let goal_position = (map[0].len() - 2, map.len() - 1);
    
    let result = simulate(
        player_pos,
        0,
        usize::MAX,
        0,
        &blizzards,
        &map,
        &mut mem,
        cycle_length,
        &Tile::End,
        &goal_position
    );
    println!("Result: {}", result.0);
}

fn part2() {
    let (map, mut blizzards) = parse_input();

    let cycle_length = get_cycle_length(&blizzards, &map);

    let mut mem: HashMap<(usize, usize, usize), (usize, usize)> = HashMap::default();
    let start_position = (1, 0);
    let end_position = (map[0].len() - 2, map.len() - 1);

    let mut sum = 0;
    for (player_pos, goal_tile, goal_position) in [
        (start_position, Tile::End, end_position),
        (end_position, Tile::Start, start_position),
        (start_position, Tile::End, end_position),
    ] {
        let (result_time, result_cycle) = simulate(
            player_pos,
            0,
            usize::MAX,
            0,
            &blizzards,
            &map,
            &mut mem,
            cycle_length,
            &goal_tile,
            &goal_position
        );

        for _ in 0..result_cycle {
            move_blizzards(&mut blizzards, &map);
        }

        println!("Time: {result_time}");

        sum += result_time;
        mem.clear();
    }

    println!("Result: {sum}");
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
