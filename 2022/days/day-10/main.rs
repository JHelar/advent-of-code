use std::env;
use std::fs;

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Noop,
    Add(i32),
}

impl Instruction {
    fn from_str(str: &str) -> Instruction {
        let mut splitted = str.split(' ');
        let instruction_str = splitted.next().unwrap();
        if instruction_str == "noop" {
            return Instruction::Noop;
        }

        let amount = splitted.next().unwrap().parse::<i32>().unwrap();
        Instruction::Add(amount)
    }
}

#[derive(Debug)]
struct CPU {
    register: i32,
    cycles: i32,
    instructions: Vec<Instruction>,
    instruction_pointer: usize,
    cycles_remaining: i32,
}

impl CPU {
    fn new(instructions: Vec<Instruction>) -> CPU {
        CPU {
            register: 1,
            cycles: 0,
            instruction_pointer: 0,
            cycles_remaining: 0,
            instructions,
        }
    }

    fn cycle(self: &mut CPU) {
        self.cycles += 1;

        if self.cycles_remaining == 0 {
            let instruction = self.instructions[self.instruction_pointer];

            match instruction {
                Instruction::Noop => {
                    self.cycles_remaining = 1;
                }
                Instruction::Add(_) => {
                    self.cycles_remaining = 2;
                }
            }
        }

        self.cycles_remaining -= 1;
        if self.cycles_remaining == 0 {
            let instruction = self.instructions[self.instruction_pointer];
            match instruction {
                Instruction::Noop => {}
                Instruction::Add(amount) => {
                    self.register += amount;
                }
            }

            self.instruction_pointer = (self.instruction_pointer + 1) % self.instructions.len();
        }
    }
}

struct CRT<'a> {
    pointer: (usize, usize),
    screen: [[&'a str; 40]; 6],
}

impl<'a> CRT<'a> {
    fn new() -> CRT<'a> {
        CRT {
            pointer: (0, 0),
            screen: [[" "; 40]; 6],
        }
    }

    fn cycle(self: &mut CRT<'a>, cpu: &CPU) {
        let pixel_value = if self.pointer.0 as i32 >= (cpu.register - 1)
            && self.pointer.0 as i32 <= (cpu.register + 1)
        {
            "🎁"
        } else {
            "🎄"
        };
        self.screen[self.pointer.1][self.pointer.0] = pixel_value;

        self.pointer.0 = (self.pointer.0 + 1) % 40;

        let y_inc = if self.pointer.0 == 0 { 1 } else { 0 };
        self.pointer.1 = (self.pointer.1 + y_inc) % 6;
    }

    fn print(self: &CRT<'a>) {
        println!("\n{}", self.screen.map(|x| x.join("")).join("\n"));
    }
}

fn parse_input() -> String {
    fs::read_to_string("input.txt").expect("Unable to read file!")
}

fn parse_instructions() -> Vec<Instruction> {
    let content = parse_input();
    content
        .lines()
        .map(|x| Instruction::from_str(x.trim()))
        .collect()
}

fn part1() {
    let instructions = parse_instructions();
    let cpu = &mut CPU::new(instructions);

    let cycle_stops = vec![20, 60, 100, 140, 180, 220];
    let mut sum = 0;

    for stop in cycle_stops {
        loop {
            cpu.cycle();
            if stop - 1 == cpu.cycles {
                let strength = cpu.register * stop;
                sum += strength;
                break;
            }
        }
    }

    println!("Result: {}", sum)
}

fn part2() {
    let instructions = parse_instructions();
    let cpu = &mut CPU::new(instructions);
    let crt = &mut CRT::new();

    for _i in 0..240 {
        crt.cycle(cpu);
        cpu.cycle();
    }
    crt.print();
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
