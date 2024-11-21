use hashbrown::{HashMap, HashSet};

type Registry = Vec<isize>;
type Instructions = Vec<isize>;

const OPCODES: [OPCode; 16] = [
    OPCode::ADDr,
    OPCode::ADDi,
    OPCode::MULr,
    OPCode::MULi,
    OPCode::BANr,
    OPCode::BANi,
    OPCode::BORr,
    OPCode::BORi,
    OPCode::SETr,
    OPCode::SETi,
    OPCode::GTir,
    OPCode::GTri,
    OPCode::GTrr,
    OPCode::EQir,
    OPCode::EQri,
    OPCode::EQrr,
];

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum OPCode {
    ADDr,
    ADDi,
    MULr,
    MULi,
    BANr,
    BANi,
    BORr,
    BORi,
    SETr,
    SETi,
    GTir,
    GTri,
    GTrr,
    EQir,
    EQri,
    EQrr,
}

impl OPCode {
    fn run(&self, a: isize, b: isize, registry: &Registry) -> isize {
        match self {
            OPCode::ADDr => {
                let a_value = registry[a as usize];
                let b_value = registry[b as usize];
                a_value + b_value
            }
            OPCode::ADDi => {
                let a_value = registry[a as usize];
                a_value + b
            }
            OPCode::MULr => {
                let a_value = registry[a as usize];
                let b_value = registry[b as usize];
                a_value * b_value
            }
            OPCode::MULi => {
                let a_value = registry[a as usize];
                a_value * b
            }
            OPCode::BANr => {
                let a_value = registry[a as usize];
                let b_value = registry[b as usize];
                a_value & b_value
            }
            OPCode::BANi => {
                let a_value = registry[a as usize];
                a_value & b
            }
            OPCode::BORr => {
                let a_value = registry[a as usize];
                let b_value = registry[b as usize];
                a_value | b_value
            }
            OPCode::BORi => {
                let a_value = registry[a as usize];
                a_value | b
            }
            OPCode::SETr => {
                let a_value = registry[a as usize];
                a_value.clone()
            }
            OPCode::SETi => a.clone(),
            OPCode::GTir => {
                let b_value = registry[b as usize];
                if a > b_value {
                    1
                } else {
                    0
                }
            }
            OPCode::GTri => {
                let a_value = registry[a as usize];
                if a_value > b {
                    1
                } else {
                    0
                }
            }
            OPCode::GTrr => {
                let a_value = registry[a as usize];
                let b_value = registry[b as usize];
                if a_value > b_value {
                    1
                } else {
                    0
                }
            }
            OPCode::EQir => {
                let b_value = registry[b as usize];
                if a == b_value {
                    1
                } else {
                    0
                }
            }
            OPCode::EQri => {
                let a_value = registry[a as usize];
                if a_value == b {
                    1
                } else {
                    0
                }
            }
            OPCode::EQrr => {
                let a_value = registry[a as usize];
                let b_value = registry[b as usize];
                if a_value == b_value {
                    1
                } else {
                    0
                }
            }
        }
    }
}

fn read_input() -> (Vec<(Registry, Instructions, Registry)>, Vec<Instructions>) {
    let mut captures: Vec<(Registry, Instructions, Registry)> = Vec::new();
    let mut instructions: Vec<Instructions> = Vec::new();

    let mut before: Option<Registry> = None;
    let mut instruction: Option<Instructions> = None;
    let mut after: Option<Registry> = None;
    let mut empty_count = 0;

    for line in std::io::stdin().lines().filter_map(|line| line.ok()) {
        if line.is_empty() {
            empty_count += 1;
            continue;
        }
        // Reading instructions
        if empty_count == 3 {
            let instruction = line
                .split(' ')
                .map(|num| num.parse::<isize>().unwrap())
                .collect();

            instructions.push(instruction);
        }
        // Reading captures
        else {
            empty_count = 0;
            if before.is_none() {
                before = Some(
                    line.replace("Before: [", "")
                        .replace("]", "")
                        .split(", ")
                        .map(|num| num.parse::<isize>().unwrap())
                        .collect(),
                );

                continue;
            }
            if instruction.is_none() {
                instruction = Some(
                    line.split(' ')
                        .map(|num| num.parse::<isize>().unwrap())
                        .collect(),
                );
                continue;
            }
            if after.is_none() {
                after = Some(
                    line.replace("After:  [", "")
                        .replace("]", "")
                        .split(", ")
                        .map(|num| num.parse::<isize>().unwrap())
                        .collect(),
                );
                captures.push((before.unwrap(), instruction.unwrap(), after.unwrap()));

                before = None;
                instruction = None;
                after = None
            }
        }
    }

    (captures, instructions)
}

fn map_op_code(
    code_index: usize,
    truth: &HashMap<isize, OPCode>,
    map: &HashMap<OPCode, HashSet<isize>>,
) -> Result<HashMap<isize, OPCode>, ()> {
    if code_index >= OPCODES.len() {
        return Ok(truth.clone());
    }

    let op_code = OPCODES[code_index];
    let potential_codes: Vec<isize> = map
        .get(&op_code)
        .unwrap()
        .iter()
        .filter_map(|potential_code| {
            if !truth.contains_key(potential_code) {
                Some(potential_code.clone())
            } else {
                None
            }
        })
        .collect();

    let mut potential_truth = truth.clone();
    for potential_code in potential_codes {
        // Test
        potential_truth.insert(potential_code.clone(), op_code.clone());
        match map_op_code(code_index + 1, &potential_truth, map) {
            Err(_) => {
                potential_truth.remove(&potential_code);
            }
            Ok(result) => return Ok(result),
        }
    }
    Err(())
}

fn part1() -> Option<isize> {
    let (captures, _) = read_input();
    let mut sum = 0;
    for (registry, instruction, output) in captures.iter() {
        let a = instruction[1];
        let b = instruction[2];
        let c = instruction[3];
        let expected = output[c as usize];
        let potential_op_codes = OPCODES
            .iter()
            .filter(|&opcode| {
                let result = opcode.run(a, b, registry);
                result == expected
            })
            .count();

        if potential_op_codes >= 3 {
            sum += 1;
        }
    }

    Some(sum)
}

fn part2() -> Option<isize> {
    let (captures, instructions) = read_input();
    let mut op_code_mapping: HashMap<OPCode, HashSet<isize>> = HashMap::new();

    for (registry, instruction, output) in captures.iter() {
        let code = instruction[0];
        let a = instruction[1];
        let b = instruction[2];
        let c = instruction[3];
        let expected = output[c as usize];
        for op_code in OPCODES.iter() {
            let result = op_code.run(a, b, registry);
            if result == expected {
                let entry = op_code_mapping.entry(*op_code).or_insert(HashSet::new());
                entry.insert(code);
            }
        }
    }

    let truth = HashMap::new();
    let code_map = map_op_code(0, &truth, &op_code_mapping).unwrap();
    let mut registry: Registry = vec![0, 0, 0, 0];

    for instruction in instructions {
        let code = code_map.get(&instruction[0]).unwrap();
        let a = instruction[1];
        let b = instruction[2];
        let c = instruction[3];

        registry[c as usize] = code.run(a, b, &registry)
    }

    Some(registry[0])
}

fn main() {
    println!("--- Day 16: Chronal Classification ---");
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
