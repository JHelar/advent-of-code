use std::{collections::HashMap, io::Read};

type Instruction = Vec<u32>;

struct HASHER {}

impl HASHER {
    fn run(set: &Instruction) -> u32 {
        set.iter().fold(0, |result, c| ((result + *c) * 17) % 256)
    }

    fn get_string(set: &Instruction) -> String {
      set.iter().filter_map(|c| char::from_u32(*c)).collect()
    }
}

struct HASHMAP {
    boxes: HashMap<u32, Vec<(String, u32)>>,
}

impl HASHMAP {
    fn new() -> Self {
        Self {
            boxes: HashMap::new(),
        }
    }

    fn add(&mut self, key: &Instruction, value: u32) {
        let hash_key = HASHER::run(key);
        let label = HASHER::get_string(key);

        if let Some(hash_box) = self.boxes.get_mut(&hash_key) {
            if let Some(box_index) = hash_box
                .iter()
                .position(|(box_label, _)| *box_label == label)
            {
                hash_box[box_index].1 = value;
            } else {
                hash_box.push((label, value));
            }
        } else {
            self.boxes.insert(hash_key, vec![(label, value)]);
        }
    }

    fn remove(&mut self, key: &Instruction) {
        let hash_key = HASHER::run(key);
        let label = HASHER::get_string(key);

        if let Some(hash_box) = self.boxes.get_mut(&hash_key) {
            if let Some(box_index) = hash_box
                .iter()
                .position(|(box_label, _)| *box_label == label)
            {
                hash_box.remove(box_index);
            }
        }
    }

    fn run(&mut self, instruction: &Instruction) {
        // 61 =
        // 45 -
        let mut key = Vec::new();
        let mut add_value_digits = Vec::new();
        let mut add_instruction = false;
        let mut remove_instruction = false;

        for c in instruction.iter() {
            if *c == 61 {
                // Add
                add_instruction = true;
            } else if *c == 45 {
                // Remove
                remove_instruction = true;
            } else {
                // Label or value
                if !add_instruction {
                    key.push(*c);
                } else {
                    add_value_digits.push(*c);
                }
            }
        }

        if add_instruction {
            self.add(&key, HASHER::get_string(&add_value_digits).parse().unwrap());
        } else if remove_instruction {
            self.remove(&key)
        } else {
            panic!("Instruction did not include operation!")
        }
    }
}

fn read_input() -> Vec<Vec<u32>> {
    std::io::stdin()
        .bytes()
        .filter_map(|byte| byte.ok())
        .fold(Vec::new(), |mut acc, c| {
            if c == 44 || acc.is_empty() {
                acc.push(Vec::new());
            }
            if c != 44 && c != 10 {
                acc.last_mut().unwrap().push(c as u32);
            }
            acc
        })
}

fn part1() -> Option<u32> {
    let hashes = read_input();
    let result = hashes.iter().map(HASHER::run).sum();

    Some(result)
}

fn part2() -> Option<u32> {
    let instructions = read_input();
    let mut map = HASHMAP::new();

    instructions.iter().for_each(|instruction| {
        map.run(instruction);
    });

    let result: u32 = map
        .boxes
        .iter()
        .map(|(box_number, hash_box)| {
            hash_box
                .iter()
                .enumerate()
                .map(|(slot_number, (_, focal_length))| {
                    (1 + *box_number) * (slot_number as u32 + 1) * *focal_length
                })
                .sum::<u32>()
        }).sum();

    Some(result)
}

fn main() {
    println!("--- Day 15: Lens Library ---");
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
