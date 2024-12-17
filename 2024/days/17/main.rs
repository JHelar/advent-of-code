#![allow(non_camel_case_types)]

use std::{fmt::Display, os::unix::process, result};

type Registry = [usize; 3];

const RA: usize = 0;
const RB: usize = 1;
const RC: usize = 2;

#[derive(Debug)]
enum Operand {
    Value(usize),
}

impl Operand {
    fn print_combo(&self) -> String {
        match self {
            Self::Value(value) => match value {
                _ if value < &4 => value.to_string(),
                4 => "RA".to_string(),
                5 => "RB".to_string(),
                6 => "RC".to_string(),
                7 => todo!("Not yet"),
                _ => panic!("Unknown combo value"),
            },
        }
    }
    fn print_literal(&self) -> String {
        match self {
            Self::Value(value) => value.to_string(),
        }
    }
    fn literal(&self) -> usize {
        match self {
            Self::Value(value) => *value,
        }
    }

    fn combo(&self, registry: &Registry) -> usize {
        match self {
            Self::Value(value) => match value {
                _ if value < &4 => *value,
                4 => registry[RA],
                5 => registry[RB],
                6 => registry[RC],
                7 => todo!("Not yet"),
                _ => panic!("Unknown combo value"),
            },
        }
    }
}

struct Computer {
    pointer: usize,
    registry: Registry,
    program: Vec<usize>,
}

impl Computer {
    fn new(program: Vec<usize>, registry: Registry) -> Self {
        Self {
            pointer: 0,
            registry,
            program,
        }
    }

    fn reset(&mut self, registry: Registry) {
        self.pointer = 0;
        self.registry = registry;
    }

    fn run(&mut self) -> Vec<usize> {
        let mut output = Vec::new();
        while let Some(code) = self.program.get(self.pointer) {
            let op_code = &OP_CODES[*code];
            let operand = Operand::Value(self.program[self.pointer + 1]);

            if let Some(value) = op_code.exec(operand, self) {
                output.push(value);
            }
        }
        output
    }

    fn print_source(&self) {
        for (line, chunk) in self.program.chunks(2).enumerate() {
            let op_code = &OP_CODES[chunk[0]];
            let operand = Operand::Value(chunk[1]);

            op_code.print(operand, line);
            print!("\n");
        }
    }

    fn run_decompiled(&mut self) -> Vec<usize> {
      let mut output = Vec::new();

      for _ in 0..self.program.len() {
        let exponent = (self.registry[RA] as i128 % 8) ^ 1;
        let pow = (2 as i128).pow(exponent as u32);
        let result = (((exponent ^ 4) ^ (self.registry[RA] as i128 / pow)) % 8) as usize;
        self.registry[RA] = self.registry[RA] / 8;

        output.push(result)
      }

      output
    }
}

const OP_CODES: [OPCode; 8] = [
    OPCode::adv,
    OPCode::bxl,
    OPCode::bst,
    OPCode::jnz,
    OPCode::bxc,
    OPCode::out,
    OPCode::bdv,
    OPCode::cdv,
];

#[derive(Debug)]
enum OPCode {
    adv,
    bxl,
    bst,
    jnz,
    bxc,
    out,
    bdv,
    cdv,
}

impl OPCode {
    fn exec(
        &self,
        operand: Operand,
        Computer {
            pointer,
            registry,
            program: _
        }: &mut Computer,
    ) -> Option<usize> {
        match self {
            OPCode::adv => {
                let denominator = (2 as i64).pow(operand.combo(registry) as u32);
                registry[RA] = ((registry[RA] as i64) / denominator) as usize;
                *pointer += 2;

                None
            }
            OPCode::bxl => {
                registry[RB] = registry[RB] ^ operand.literal();
                *pointer += 2;

                None
            }
            OPCode::bst => {
                registry[RB] = operand.combo(registry) % 8;
                *pointer += 2;

                None
            }
            OPCode::jnz => {
                if registry[RA] != 0 {
                    *pointer = operand.literal()
                } else {
                    *pointer += 2;
                }

                None
            }
            OPCode::bxc => {
                registry[RB] = registry[RB] ^ registry[RC];
                *pointer += 2;

                None
            }
            OPCode::out => {
                *pointer += 2;

                Some(operand.combo(registry) % 8)
            }
            OPCode::bdv => {
                let denominator = (2 as i64).pow(operand.combo(registry) as u32);
                registry[RB] = ((registry[RA] as i64) / denominator) as usize;
                *pointer += 2;

                None
            }
            OPCode::cdv => {
                let denominator = (2 as i64).pow(operand.combo(registry) as u32);
                registry[RC] = ((registry[RA] as i64) / denominator) as usize;
                *pointer += 2;

                None
            }
        }
    }

    fn print(&self, operand: Operand, line: usize) {
        match self {
            OPCode::adv => {
                print!("{line}: RA = RA / 2^{}", operand.print_combo())
            }
            OPCode::bxl => {
                print!("{line}: RB = RB ^ {}", operand.print_literal())
            }
            OPCode::bst => {
                print!("{line}: RB = {} % 8", operand.print_combo())
            }
            OPCode::jnz => {
                print!("{line}: if RA != 0 {{ JUMP {} }}", operand.print_literal())
            }
            OPCode::bxc => {
                print!("{line}: RB = RB ^ RC")
            }
            OPCode::out => {
                print!("{line}: OUT = {} % 8", operand.print_combo())
            }
            OPCode::bdv => {
                print!("{line}: RB = RA / 2^{}", operand.print_combo())
            }
            OPCode::cdv => {
                print!("{line}: RC = RA / 2^{}", operand.print_combo())
            }
        }
    }
}

fn read_input() -> Computer {
    let mut lines = std::io::stdin()
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| line.trim().to_string());

    let ra_fill_str = lines.next().unwrap();
    let ra_str = ra_fill_str.split_once(": ").unwrap().1;

    let rb_full_str = lines.next().unwrap();
    let rb_str = rb_full_str.split_once(": ").unwrap().1;

    let rc_full_str = lines.next().unwrap();
    let rc_str = rc_full_str.split_once(": ").unwrap().1;

    lines.next().unwrap();

    let program_full_str = lines.next().unwrap();
    let program_str = program_full_str.split_once(": ").unwrap().1;

    let program = program_str
        .split(",")
        .map(|digit| digit.parse().unwrap())
        .collect();

    Computer::new(
        program,
        [
            ra_str.parse().unwrap(),
            rb_str.parse().unwrap(),
            rc_str.parse().unwrap(),
        ],
    )
}

fn part1() -> Option<String> {
    let mut computer = read_input();

    let output = computer
        .run()
        .into_iter()
        .map(|o| o.to_string())
        .collect::<Vec<String>>();
    let output_str = output.join(",");
    Some(output_str)
}

fn part2() -> Option<String> {
    let mut computer = read_input();

    // Print source code
    computer.print_source();

    /*
     0: RB = RA % 8
     1: RB = RB ^ 1
     2: RC = RA / 2^RB
     3: RA = RA / 2^3
     4: RB = RB ^ 4
     5: RB = RB ^ RC
     6: OUT = RB % 8
     7: if RA != 0 { JUMP 0 }
    */

    // Can be simplified to
    // while(RA != 0) {
    //   OUT = ((((RA % 8) ^ 1) ^ 4) ^ (RA / 2^((RA % 8) ^ 1))) % 8
    //   RA = RA / 8
    //   }
    // For every three bits (111) the out value is calculated with three bits being right shifted for each new calculation (RA = RA / 8)
    // The program is 16 * 3 => 48bits in length, find the last 8 outputs, shift the result 8 * 3 => 24 bits to get the start of the search for the first 8 outputs which resides in the first 24 bits

    // Search for the ra that satisfies the last 8 output values (contained in 8*3bits)
    for last_eight in (0..16777215).filter(|&ra| {
        computer.reset([ra, 0, 0]);
        let output = computer.run();
        output == computer.program[8..]
    }).collect::<Vec<usize>>() {
        // The search space starts at last_eight << 24
        let ra_start = last_eight << 24;
        if let Some(result) = (ra_start..=ra_start + 16777215).find(|&ra| {
            computer.reset([ra, 0, 0]);
            let output = computer.run();
            output == computer.program
        }) {
            return Some(result.to_string());
        }
    }
    None
}

fn main() {
    println!("--- Day 17: Chronospatial Computer ---");
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
