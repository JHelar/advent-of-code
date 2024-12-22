use std::usize;

use hashbrown::HashMap;

use graph::{bfs, Edge, Graph, Node, NodeRef, NodeValue};
use vector2::{Vector2, DOWN, LEFT, RIGHT, UP, ZERO};

mod graph;
mod vector2;

type TileSequence = Vec<Tile>;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Tile {
    Number(u8, Vector2),
    Up(Vector2),
    Down(Vector2),
    Left(Vector2),
    Right(Vector2),
    Activate(Vector2),
    Gap,
}

impl Tile {
    fn get_position(&self) -> &Vector2 {
        match self {
            Tile::Number(_, position) => position,
            Tile::Activate(position) => position,
            Tile::Up(position) => position,
            Tile::Down(position) => position,
            Tile::Left(position) => position,
            Tile::Right(position) => position,
            _ => panic!("Tile does not have a position"),
        }
    }
}

impl NodeValue for Tile {
    fn to_name(&self) -> String {
        match self {
            Tile::Number(number, _) => format!("{number}"),
            Tile::Up(_) => "^".to_string(),
            Tile::Down(_) => "v".to_string(),
            Tile::Left(_) => "<".to_string(),
            Tile::Right(_) => ">".to_string(),
            Tile::Activate(_) => "A".to_string(),
            Tile::Gap => "G".to_string(),
        }
    }
}

const NUMERIC_KEYPAD: [[Tile; 3]; 4] = [
    [
        Tile::Number(7, Vector2(0, 0)),
        Tile::Number(8, Vector2(1, 0)),
        Tile::Number(9, Vector2(2, 0)),
    ],
    [
        Tile::Number(4, Vector2(0, 1)),
        Tile::Number(5, Vector2(1, 1)),
        Tile::Number(6, Vector2(2, 1)),
    ],
    [
        Tile::Number(1, Vector2(0, 2)),
        Tile::Number(2, Vector2(1, 2)),
        Tile::Number(3, Vector2(2, 2)),
    ],
    [
        Tile::Gap,
        Tile::Number(0, Vector2(1, 3)),
        Tile::Activate(Vector2(2, 3)),
    ],
];

const ARROW_KEYPAD: [[Tile; 3]; 2] = [
    [
        Tile::Gap,
        Tile::Up(Vector2(1, 0)),
        Tile::Activate(Vector2(2, 0)),
    ],
    [
        Tile::Left(Vector2(0, 1)),
        Tile::Down(Vector2(1, 1)),
        Tile::Right(Vector2(2, 1)),
    ],
];

const NUMERIC_KEYPAD_TYPE: usize = 0;
const ARROW_KEYPAD_TYPE: usize = 1;

struct Robot {
    keypads: [Graph<Tile>; 2],
    keypad_mem: HashMap<(usize, Tile, Tile), usize>,
    mem: HashMap<(usize, u64), usize>,
}

impl Robot {
    fn new() -> Self {
        let numeric_keypad = {
            let nodes: HashMap<String, NodeRef<Tile>> = NUMERIC_KEYPAD
                .iter()
                .flat_map(|row| {
                    row.iter()
                        .map(|tile| (tile.to_name(), Node::new_ref(tile.clone())))
                })
                .collect();
            let edges = [
                Edge::new(
                    nodes.get(&"7".to_string()).unwrap(),
                    nodes.get(&"8".to_string()).unwrap(),
                ),
                Edge::new(
                    nodes.get(&"7".to_string()).unwrap(),
                    nodes.get(&"4".to_string()).unwrap(),
                ),
                Edge::new(
                    nodes.get(&"8".to_string()).unwrap(),
                    nodes.get(&"7".to_string()).unwrap(),
                ),
                Edge::new(
                    nodes.get(&"8".to_string()).unwrap(),
                    nodes.get(&"9".to_string()).unwrap(),
                ),
                Edge::new(
                    nodes.get(&"8".to_string()).unwrap(),
                    nodes.get(&"5".to_string()).unwrap(),
                ),
                Edge::new(
                    nodes.get(&"9".to_string()).unwrap(),
                    nodes.get(&"8".to_string()).unwrap(),
                ),
                Edge::new(
                    nodes.get(&"9".to_string()).unwrap(),
                    nodes.get(&"6".to_string()).unwrap(),
                ),
                Edge::new(
                    nodes.get(&"4".to_string()).unwrap(),
                    nodes.get(&"7".to_string()).unwrap(),
                ),
                Edge::new(
                    nodes.get(&"4".to_string()).unwrap(),
                    nodes.get(&"5".to_string()).unwrap(),
                ),
                Edge::new(
                    nodes.get(&"4".to_string()).unwrap(),
                    nodes.get(&"1".to_string()).unwrap(),
                ),
                Edge::new(
                    nodes.get(&"5".to_string()).unwrap(),
                    nodes.get(&"4".to_string()).unwrap(),
                ),
                Edge::new(
                    nodes.get(&"5".to_string()).unwrap(),
                    nodes.get(&"8".to_string()).unwrap(),
                ),
                Edge::new(
                    nodes.get(&"5".to_string()).unwrap(),
                    nodes.get(&"6".to_string()).unwrap(),
                ),
                Edge::new(
                    nodes.get(&"5".to_string()).unwrap(),
                    nodes.get(&"2".to_string()).unwrap(),
                ),
                Edge::new(
                    nodes.get(&"6".to_string()).unwrap(),
                    nodes.get(&"5".to_string()).unwrap(),
                ),
                Edge::new(
                    nodes.get(&"6".to_string()).unwrap(),
                    nodes.get(&"3".to_string()).unwrap(),
                ),
                Edge::new(
                    nodes.get(&"6".to_string()).unwrap(),
                    nodes.get(&"9".to_string()).unwrap(),
                ),
                Edge::new(
                    nodes.get(&"1".to_string()).unwrap(),
                    nodes.get(&"4".to_string()).unwrap(),
                ),
                Edge::new(
                    nodes.get(&"1".to_string()).unwrap(),
                    nodes.get(&"2".to_string()).unwrap(),
                ),
                Edge::new(
                    nodes.get(&"2".to_string()).unwrap(),
                    nodes.get(&"1".to_string()).unwrap(),
                ),
                Edge::new(
                    nodes.get(&"2".to_string()).unwrap(),
                    nodes.get(&"5".to_string()).unwrap(),
                ),
                Edge::new(
                    nodes.get(&"2".to_string()).unwrap(),
                    nodes.get(&"3".to_string()).unwrap(),
                ),
                Edge::new(
                    nodes.get(&"2".to_string()).unwrap(),
                    nodes.get(&"0".to_string()).unwrap(),
                ),
                Edge::new(
                    nodes.get(&"3".to_string()).unwrap(),
                    nodes.get(&"2".to_string()).unwrap(),
                ),
                Edge::new(
                    nodes.get(&"3".to_string()).unwrap(),
                    nodes.get(&"6".to_string()).unwrap(),
                ),
                Edge::new(
                    nodes.get(&"3".to_string()).unwrap(),
                    nodes.get(&"A".to_string()).unwrap(),
                ),
                Edge::new(
                    nodes.get(&"0".to_string()).unwrap(),
                    nodes.get(&"2".to_string()).unwrap(),
                ),
                Edge::new(
                    nodes.get(&"0".to_string()).unwrap(),
                    nodes.get(&"A".to_string()).unwrap(),
                ),
                Edge::new(
                    nodes.get(&"A".to_string()).unwrap(),
                    nodes.get(&"0".to_string()).unwrap(),
                ),
                Edge::new(
                    nodes.get(&"A".to_string()).unwrap(),
                    nodes.get(&"3".to_string()).unwrap(),
                ),
            ]
            .to_vec();

            Graph::new(nodes, edges)
        };
        let arrow_keypad = {
            let nodes: HashMap<String, NodeRef<Tile>> = ARROW_KEYPAD
                .iter()
                .flat_map(|row| {
                    row.iter()
                        .map(|tile| (tile.to_name(), Node::new_ref(tile.clone())))
                })
                .collect();
            let edges = [
                Edge::new(
                    nodes.get(&"^".to_string()).unwrap(),
                    nodes.get(&"A".to_string()).unwrap(),
                ),
                Edge::new(
                    nodes.get(&"^".to_string()).unwrap(),
                    nodes.get(&"v".to_string()).unwrap(),
                ),
                Edge::new(
                    nodes.get(&"A".to_string()).unwrap(),
                    nodes.get(&"^".to_string()).unwrap(),
                ),
                Edge::new(
                    nodes.get(&"A".to_string()).unwrap(),
                    nodes.get(&">".to_string()).unwrap(),
                ),
                Edge::new(
                    nodes.get(&"<".to_string()).unwrap(),
                    nodes.get(&"v".to_string()).unwrap(),
                ),
                Edge::new(
                    nodes.get(&"v".to_string()).unwrap(),
                    nodes.get(&"<".to_string()).unwrap(),
                ),
                Edge::new(
                    nodes.get(&"v".to_string()).unwrap(),
                    nodes.get(&"^".to_string()).unwrap(),
                ),
                Edge::new(
                    nodes.get(&"v".to_string()).unwrap(),
                    nodes.get(&">".to_string()).unwrap(),
                ),
                Edge::new(
                    nodes.get(&">".to_string()).unwrap(),
                    nodes.get(&"v".to_string()).unwrap(),
                ),
                Edge::new(
                    nodes.get(&">".to_string()).unwrap(),
                    nodes.get(&"A".to_string()).unwrap(),
                ),
            ]
            .to_vec();

            Graph::new(nodes, edges)
        };

        Self {
            keypads: [numeric_keypad, arrow_keypad],
            mem: HashMap::new(),
            keypad_mem: HashMap::new(),
        }
    }

    fn tile_path_to_sequence(&self, start: &Vector2, path: &Vec<Tile>) -> TileSequence {
        let mut current_position = *start;
        let mut arm_sequence = path
            .iter()
            .map(|tile| {
                let next_position = tile.get_position();
                let direction_tile = match current_position.get_direction(next_position) {
                    UP => Tile::Up,
                    DOWN => Tile::Down,
                    LEFT => Tile::Left,
                    RIGHT => Tile::Right,
                    direction => panic!(
                        "Unknown direction: {direction}, {current_position} => {next_position}"
                    ),
                };
                current_position = next_position.clone();
                direction_tile(ZERO)
            })
            .collect::<TileSequence>();

        arm_sequence.push(Tile::Activate(ZERO));
        arm_sequence
    }

    fn run(&mut self, sequence: &TileSequence, keypad_type: usize, depth: usize) -> usize {
        if let Some(&result_length) = self.mem.get(&(depth, Robot::hash_sequence(sequence))) {
            return result_length;
        }

        if depth == 0 {
            return sequence.len();
        }

        let mut arm_position = self.keypads[keypad_type]
            .nodes
            .get("A")
            .unwrap()
            .lock()
            .unwrap()
            .value
            .clone();

        let subsequences = sequence.split_inclusive(|tile| matches!(tile, Tile::Activate(_)));
        let mut total_length = 0;

        for subsequence in subsequences {
            let subsequence_hash = Robot::hash_sequence(&subsequence.to_vec());

            if let Some(&result_length) = self.mem.get(&(depth, subsequence_hash)) {
                total_length += result_length;
            } else {
                let mut subsequence_length = 0;
                for goal in subsequence {
                    let best_path_length = if let Some(&best_length) =
                        self.keypad_mem
                            .get(&(depth, arm_position.clone(), goal.clone()))
                    {
                        best_length
                    } else {
                        let mut paths = bfs::get_paths(
                            &arm_position,
                            goal,
                            |edge| !matches!(edge.get_destination().value, Tile::Gap),
                            usize::MAX,
                            &self.keypads[keypad_type],
                        );

                        paths.sort_by(|a, b| a.len().cmp(&b.len()));
                        let min_len = paths[0].len();

                        let mut best_len = usize::MAX;

                        for path in paths.iter().take_while(|p| p.len() == min_len) {
                            let tile_path: Vec<Tile> = path
                                .iter()
                                .map(|node| node.lock().unwrap().value.clone())
                                .collect();

                            let path_sequence = self
                                .tile_path_to_sequence(&arm_position.get_position(), &tile_path);

                            let completed_length =
                                self.run(&path_sequence, ARROW_KEYPAD_TYPE, depth - 1);
                            if completed_length < best_len {
                                best_len = completed_length;
                            }
                        }

                        self.keypad_mem
                            .insert((depth, arm_position.clone(), goal.clone()), best_len);
                        best_len
                    };

                    arm_position = self.keypads[keypad_type]
                        .nodes
                        .get(&goal.to_name())
                        .unwrap()
                        .lock()
                        .unwrap()
                        .value
                        .clone();

                    subsequence_length += best_path_length;
                }

                self.mem
                    .insert((depth, subsequence_hash), subsequence_length);

                total_length += subsequence_length;
            }
        }

        self.mem
            .insert((depth, Robot::hash_sequence(sequence)), total_length);
        total_length
    }

    fn hash_sequence(sequence: &TileSequence) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        sequence.hash(&mut hasher);
        hasher.finish()
    }
}

fn sequence_to_string(sequence: &TileSequence) -> String {
    format!(
        "{} => {}",
        sequence.len(),
        sequence
            .iter()
            .map(|tile| tile.to_name())
            .collect::<String>(),
    )
}

fn read_input() -> Vec<TileSequence> {
    std::io::stdin()
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            line.chars()
                .into_iter()
                .map(|c| match c {
                    'A' => Tile::Activate(ZERO),
                    _ if c.is_digit(10) => Tile::Number(c.to_digit(10).unwrap() as u8, ZERO),
                    _ => panic!("Unknown number"),
                })
                .collect()
        })
        .collect()
}

fn part1() -> Option<isize> {
    let sequences = read_input();

    let mut robot = Robot::new();
    let mut sum = 0;
    for sequence in sequences.iter() {
        let result_sequence = robot.run(sequence, NUMERIC_KEYPAD_TYPE, 3);

        let sequence_value_string: String = sequence
            .iter()
            .filter_map(|tile| match tile {
                Tile::Number(value, _) => Some(value.to_string()),
                _ => None,
            })
            .collect();
        let sequence_value: isize = sequence_value_string.parse().unwrap();
        let result = sequence_value * result_sequence as isize;
        println!(
            "{} * {} = {}",
            result_sequence,
            sequence_value,
            result
        );
        sum += result
    }

    Some(sum)
}

fn part2() -> Option<isize> {
    let sequences = read_input();

    let mut robot = Robot::new();
    let mut sum = 0;
    for sequence in sequences.iter() {
        let result_sequence = robot.run(sequence, NUMERIC_KEYPAD_TYPE, 26);

        let sequence_value_string: String = sequence
            .iter()
            .filter_map(|tile| match tile {
                Tile::Number(value, _) => Some(value.to_string()),
                _ => None,
            })
            .collect();
        let sequence_value: isize = sequence_value_string.parse().unwrap();
        let result = sequence_value * result_sequence as isize;
        println!(
            "{} * {} = {}",
            result_sequence,
            sequence_value,
            result
        );
        sum += result
    }

    Some(sum)
}

fn main() {
    println!("--- Day 21: Keypad Conundrum ---");
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
