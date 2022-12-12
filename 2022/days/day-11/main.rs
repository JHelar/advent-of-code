use std::collections::HashMap;
use std::env;
use std::fs;

#[derive(Debug, Clone, Copy)]
enum OpSign {
    Plus,
    Multiplied,
}

#[derive(Debug, Clone, Copy)]
enum OpValue {
    Num(i128),
    Old
}

#[derive(Debug, Clone, Copy)]
struct Operation(OpSign, OpValue);

impl Operation {
    fn calculate(self: &Operation, value: i128) -> i128 {
        match self.1 {
            OpValue::Num(num) => {
                match self.0 {
                    OpSign::Plus => {
                        value + num
                    },
                    OpSign::Multiplied => {
                        value * num
                    }
                }
            },
            OpValue::Old => {
                match self.0 {
                    OpSign::Plus => {
                        value + value
                    },
                    OpSign::Multiplied => {
                        (value * value) as i128
                    }
                }
            }
        }
    }
}

impl OpValue {
    fn from_str(str: &str) -> OpValue {
        if str == "old" {
            return OpValue::Old;
        }
        OpValue::Num(str.parse::<i128>().unwrap())
    }
}

impl OpSign {
    fn from_str(str: &str) -> OpSign {
        if str == "+" {
            return OpSign::Plus;
        }
        OpSign::Multiplied
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<i128>,
    operation: Operation,
    test_value: i128,
    true_monkey: usize,
    false_monkey: usize,
    ispections: i128
}

impl Monkey {
    fn operate(self: &mut Monkey, relief: i128, cycle_length: i128) -> Vec<(usize, i128)> {
        let mut result: Vec<(usize, i128)> = vec![];

        while let Some(item) = self.items.pop() {
            self.ispections += 1;
            let mut worry = self.operation.calculate(item) / relief;
            if cycle_length > 0 {
                worry = worry % cycle_length;
            }

            if worry % self.test_value == 0 {
                result.push((self.true_monkey, worry));
            } else {
                result.push((self.false_monkey, worry));
            }
        }

        result
    }
}

fn get_monkey_cycle(monkeys: &Vec<Monkey>) -> i128 {
    let mut divisors: Vec<i128> = monkeys.iter().map(|monkey| monkey.test_value).collect();
    divisors.sort();
    divisors.reverse();

    let mut cycle = 0;
    let mut initial_pattern = String::new();
    loop {
        let mut pattern = String::new();
        
        for divisor in divisors.iter() {
            if cycle % divisor == 0 {
                pattern.push_str(format!("{}", divisor).as_str());
            }
        }

        if cycle == 0 {
            initial_pattern = pattern.clone();
        } else {
            if initial_pattern == pattern {
                return cycle;
            }
        }

        cycle += 1;
    }

    // for val in 0..=(divisors[0] * 20000) {
    //     let mut divisibles: Vec<i128> = Vec::new();
        
    //     for divisor in divisors.iter() {
    //         if val % divisor == 0 {
    //             divisibles.push(*divisor);
    //         }
    //     }

    //     divisor_map.insert(val, divisibles);
    // }

    // let mut lines: Vec<String> = Vec::new();
    // for val in 0..=(divisors[0] * 20000) {
    //     lines.push(format!("{:?}", divisor_map[&val]));
    // }
    // fs::write("test.txt", lines.join("\n"));
}

fn parse_input() -> String {
    fs::read_to_string("input.txt").expect("Unable to read file!")
}

fn parse_monkeys() -> Vec<Monkey> {
    let content = parse_input();
    content
        .split("\n\n")
        .map(|monkey_line| {
            let mut value_lines = monkey_line.trim().lines();

            let _ = value_lines.next().unwrap();

            let items = value_lines
                .next()
                .unwrap()
                .split(": ")
                .last()
                .unwrap()
                .split(", ")
                .map(|vale_str| vale_str.parse::<i128>().unwrap())
                .collect();

            let operation_str = value_lines.next().unwrap().split("old ").last().unwrap();
            let operetation_sign = OpSign::from_str(&operation_str[0..1]);
            let operetation_value = OpValue::from_str(&operation_str[2..]);

            let test_value = value_lines
                .next()
                .unwrap()
                .split("by ")
                .last()
                .unwrap()
                .parse::<i128>()
                .unwrap();

            let true_monkey_value = value_lines
                .next()
                .unwrap()
                .split("monkey ")
                .last()
                .unwrap()
                .parse::<usize>()
                .unwrap();

            let false_monkey_value = value_lines
                .next()
                .unwrap()
                .split("monkey ")
                .last()
                .unwrap()
                .parse::<usize>()
                .unwrap();

            Monkey {
                items,
                operation: Operation(operetation_sign, operetation_value),
                test_value,
                true_monkey: true_monkey_value,
                false_monkey: false_monkey_value,
                ispections: 0
            }
        })
        .collect()
}

fn part1() {
    let monkeys = &mut parse_monkeys();
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let monkey_result = monkeys[i].operate(3, -1);
            for (monkey_index, item) in monkey_result {
                monkeys[monkey_index].items.push(item);
            }
        }
    }

    let mut inspections = monkeys.iter().map(|monkey| monkey.ispections).collect::<Vec<i128>>();
    inspections.sort();
    inspections.reverse();

    let result = inspections[0] * inspections[1];
    println!("Result: {}", result);
}

fn part2() {
    let monkeys = &mut parse_monkeys();
    let monkey_cycle = get_monkey_cycle(monkeys);
    println!("Cycle length: {}", monkey_cycle);

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            let monkey_result = monkeys[i].operate(1, monkey_cycle);
            for (monkey_index, item) in monkey_result {
                monkeys[monkey_index].items.push(item);
            }
        }
    }

    let mut inspections = monkeys.iter().map(|monkey| monkey.ispections).collect::<Vec<i128>>();
    inspections.sort();
    inspections.reverse();

    let result = inspections[0] * inspections[1];
    println!("Result: {}", result);
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
