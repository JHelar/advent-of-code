use regex::Regex;

#[derive(Debug)]
enum Instruction {
    Mul(isize, isize),
}

impl Instruction {
    fn exec(instruction: Instruction) -> isize {
        match instruction {
            Self::Mul(a, b) => a * b,
        }
    }
}

fn read_input(mul_override: bool) -> impl Iterator<Item = Instruction> {
    let mut is_enabled = true;
    let instruction_regex =
        Regex::new(r"(mul\(([\d]+),([\d]+)\)|do\(()()\)|don\'t\(()()\))").unwrap();

    std::io::stdin()
        .lines()
        .filter_map(|line| line.ok())
        .flat_map(move |line| {
            instruction_regex
                .captures_iter(&line.clone())
                .map(|c| c.extract())
                .filter_map(move |(_, [instruction, a, b])| match instruction {
                    "do()" => {
                        is_enabled = true;
                        None
                    }
                    "don't()" => {
                        is_enabled = false;
                        None
                    }
                    _ if is_enabled || mul_override => {
                        Some(Instruction::Mul(a.parse().unwrap(), b.parse().unwrap()))
                    }
                    _ => None,
                })
                .collect::<Vec<Instruction>>()
        })
}

fn part1() -> Option<isize> {
    let instructions = read_input(true);
    Some(instructions.map(Instruction::exec).sum())
}

fn part2() -> Option<isize> {
    let instructions = read_input(false);
    Some(instructions.map(Instruction::exec).sum())
}

fn main() {
    println!("--- Day 3: Mull It Over ---");
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
