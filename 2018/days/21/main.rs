use std::collections::{HashMap, HashSet};

use phf::phf_map;

type Register = [isize; 6];
type Instruction = (OPCode, isize, isize, isize);

static OP_MAP: phf::Map<&'static str, OPCode> = phf_map! {
  "addr" => OPCode::ADDr,
  "addi" => OPCode::ADDi,
  "mulr" => OPCode::MULr,
  "muli" => OPCode::MULi,
  "banr" => OPCode::BANr,
  "bani" => OPCode::BANi,
  "borr" => OPCode::BORr,
  "bori" => OPCode::BORi,
  "setr" => OPCode::SETr,
  "seti" => OPCode::SETi,
  "gtir" => OPCode::GTir,
  "gtri" => OPCode::GTri,
  "gtrr" => OPCode::GTrr,
  "eqir" => OPCode::EQir,
  "eqri" => OPCode::EQri,
  "eqrr" => OPCode::EQrr,
};

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
    fn run(&self, a: isize, b: isize, register: &Register) -> isize {
        match self {
            OPCode::ADDr => {
                let a_value = register[a as usize];
                let b_value = register[b as usize];
                a_value + b_value
            }
            OPCode::ADDi => {
                let a_value = register[a as usize];
                a_value + b
            }
            OPCode::MULr => {
                let a_value = register[a as usize];
                let b_value = register[b as usize];
                a_value * b_value
            }
            OPCode::MULi => {
                let a_value = register[a as usize];
                a_value * b
            }
            OPCode::BANr => {
                let a_value = register[a as usize];
                let b_value = register[b as usize];
                a_value & b_value
            }
            OPCode::BANi => {
                let a_value = register[a as usize];
                a_value & b
            }
            OPCode::BORr => {
                let a_value = register[a as usize];
                let b_value = register[b as usize];
                a_value | b_value
            }
            OPCode::BORi => {
                let a_value = register[a as usize];
                a_value | b
            }
            OPCode::SETr => {
                let a_value = register[a as usize];
                a_value.clone()
            }
            OPCode::SETi => a.clone(),
            OPCode::GTir => {
                let b_value = register[b as usize];
                if a > b_value {
                    1
                } else {
                    0
                }
            }
            OPCode::GTri => {
                let a_value = register[a as usize];
                if a_value > b {
                    1
                } else {
                    0
                }
            }
            OPCode::GTrr => {
                let a_value = register[a as usize];
                let b_value = register[b as usize];
                if a_value > b_value {
                    1
                } else {
                    0
                }
            }
            OPCode::EQir => {
                let b_value = register[b as usize];
                if a == b_value {
                    1
                } else {
                    0
                }
            }
            OPCode::EQri => {
                let a_value = register[a as usize];
                if a_value == b {
                    1
                } else {
                    0
                }
            }
            OPCode::EQrr => {
                let a_value = register[a as usize];
                let b_value = register[b as usize];
                if a_value == b_value {
                    1
                } else {
                    0
                }
            }
        }
    }
}

fn print_instruction(ip: usize, (code, a, b, c): &Instruction) {
    match code {
        OPCode::ADDr => {
            println!("{ip}: R{c} = R{a} + R{b}")
        }
        OPCode::ADDi => {
            println!("{ip}: R{c} = R{a} + {b}")
        }
        OPCode::MULr => {
            println!("{ip}: R{c} = R{a} * R{b}")
        }
        OPCode::MULi => {
            println!("{ip}: R{c} = R{a} * {b}")
        }
        OPCode::BANr => {
            println!("{ip}: R{c} = R{a} & R{b}")
        }
        OPCode::BANi => {
            println!("{ip}: R{c} = R{a} & {b}")
        }
        OPCode::BORr => {
            println!("{ip}: R{c} = R{a} | R{b}")
        }
        OPCode::BORi => {
            println!("{ip}: R{c} = R{a} | {b}")
        }
        OPCode::SETr => {
            println!("{ip}: R{c} = R{a}")
        }
        OPCode::SETi => println!("{ip}: R{c} = {a}"),
        OPCode::GTir => {
            println!("{ip}: R{c} = {a} > R{b} ? 1 : 0")
        }
        OPCode::GTri => {
            println!("{ip}: R{c} = R{a} > {b} ? 1 : 0")
        }
        OPCode::GTrr => {
            println!("{ip}: R{c} = R{a} > R{b} ? 1 : 0")
        }
        OPCode::EQir => {
            println!("{ip}: R{c} = {a} == R{b} ? 1 : 0")
        }
        OPCode::EQri => {
            println!("{ip}: R{c} = R{a} == {b} ? 1 : 0")
        }
        OPCode::EQrr => {
            println!("{ip}: R{c} = R{a} == R{b} ? 1 : 0")
        }
    }
}

fn read_input() -> (usize, Vec<Instruction>) {
    let mut ip_line = String::new();

    let _ = std::io::stdin().read_line(&mut ip_line);
    let ip = ip_line.trim().replace("#ip ", "").parse().unwrap();

    let instructions = std::io::stdin()
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            let mut parts = line.trim().split_whitespace();
            let code_str = parts.next().unwrap();

            let code = *OP_MAP.get(code_str).unwrap();
            let a = parts.next().unwrap().parse().unwrap();
            let b = parts.next().unwrap().parse().unwrap();
            let c = parts.next().unwrap().parse().unwrap();

            (code, a, b, c)
        })
        .collect();

    (ip, instructions)
}

fn part1() -> Option<isize> {
    let (ip, instructions) = read_input();
    let mut register: Register = [0, 0, 0, 0, 0, 0];

    let mut instruction_pointer: usize = 0;
    while let Some((op_code, a, b, c)) = instructions.get(instruction_pointer) {
        // Ad int 16 r5 contains the result
        if instruction_pointer == 16 {
            return Some(register[5]);
        }
        register[ip] = instruction_pointer as isize;
        register[*c as usize] = op_code.run(*a, *b, &register);
        instruction_pointer = register[ip] as usize + 1;
    }

    None

    // for (ip, instruction) in instructions.iter().enumerate() {
    //     print_instruction(ip, instruction);
    // }

    // decompiled version:
    // let mut r0: isize = 0;
    // let mut r1: isize = 0;
    // let mut r2: isize = 0;
    // let mut r3: isize = 0;
    // let mut r5: isize = 123;
    // loop {
    //     r5 = r5 & 456;
    //     if r5 == 72 {
    //       break;
    //     }
    // }

    // r5 = 0;
    // 'outer: loop {
    //     r3 = r5 | 65536;
    //     r5 = 10828530;

    //     loop {
    //         r2 = r3 & 255;
    //         r5 = r5 + r2;
    //         r5 = r5 & 16777215;
    //         r5 = r5 * 65899;
    //         r5 = r5 & 16777215;

    //         if 256 > r3 {
    //             if r5 == r0 {
    //                  break 'outer;
    //             }
    //             break;
    //         }

    //         r2 = 0;
    //         loop {
    //             r1 = r2 + 1;
    //             r1 = r1 * 256;

    //             if r1 > r3 {
    //                 r3 = r2;
    //                 break;
    //             } else {
    //                 r2 += 1;
    //             }
    //         }
    //     }
    // }
}

fn part2() -> Option<isize> {
    let (ip, instructions) = read_input();
    let mut register: Register = [0, 0, 0, 0, 0, 0];

    let mut instruction_pointer: usize = 0;
    let mut prev_value = 0;
    let mut mem = HashSet::new();

    while let Some((op_code, a, b, c)) = instructions.get(instruction_pointer) {
        if instruction_pointer == 16 {
            if !mem.insert(register[5]) {
                return Some(prev_value);
            }
            prev_value = register[5];
        }
        register[ip] = instruction_pointer as isize;
        register[*c as usize] = op_code.run(*a, *b, &register);
        instruction_pointer = register[ip] as usize + 1;
    }

    None
}

fn main() {
    println!("--- Day 21: Chronal Conversion ---");
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
