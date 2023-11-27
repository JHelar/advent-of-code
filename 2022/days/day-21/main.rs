use std::collections::HashMap;
use std::env;
use std::fs;

#[derive(Debug, Clone)]
enum Monkey {
    Num(i64),
    Add(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
}

impl Monkey {
    fn get_value(
        monkey_name: &String,
        all_monkeys: &HashMap<String, Monkey>,
        mem: &mut HashMap<String, i64>,
    ) -> i64 {
        let monkey = all_monkeys.get(monkey_name).unwrap();
        let res = match monkey {
            Self::Num(value) => *value,
            Self::Add(name1, name2) => {
                let m1 = Self::get_value(name1, all_monkeys, mem);
                let m2 = Self::get_value(name2, all_monkeys, mem);

                m1 + m2
            }
            Self::Div(name1, name2) => {
                let m1 = Self::get_value(name1, all_monkeys, mem);
                let m2 = Self::get_value(name2, all_monkeys, mem);

                m1 / m2
            }
            Self::Mul(name1, name2) => {
                let m1 = Self::get_value(name1, all_monkeys, mem);
                let m2 = Self::get_value(name2, all_monkeys, mem);

                m1 * m2
            }
            Self::Sub(name1, name2) => {
                let m1 = Self::get_value(name1, all_monkeys, mem);
                let m2 = Self::get_value(name2, all_monkeys, mem);

                m1 - m2
            }
        };
        mem.insert(monkey_name.clone(), res);
        res
    }

    fn find_monkey_path(
        from_monkey_name: &String,
        monkey_name: &String,
        all_monkeys: &HashMap<String, Monkey>,
    ) -> Option<Vec<String>> {
        let monkey = all_monkeys.get(from_monkey_name).unwrap();
        if from_monkey_name == monkey_name {
            return Some(vec![]);
        }
        match monkey {
            Self::Num(_) => None,
            Self::Add(name1, name2) => {
                if let Some(mut path) = Self::find_monkey_path(name1, monkey_name, all_monkeys) {
                    path.push(name1.clone());
                    return Some(path);
                }

                if let Some(mut path) = Self::find_monkey_path(name2, monkey_name, all_monkeys) {
                    path.push(name2.clone());
                    return Some(path);
                }

                None
            }
            Self::Div(name1, name2) => {
                if let Some(mut path) = Self::find_monkey_path(name1, monkey_name, all_monkeys) {
                    path.push(name1.clone());
                    return Some(path);
                }

                if let Some(mut path) = Self::find_monkey_path(name2, monkey_name, all_monkeys) {
                    path.push(name2.clone());
                    return Some(path);
                }

                None
            }
            Self::Mul(name1, name2) => {
                if let Some(mut path) = Self::find_monkey_path(name1, monkey_name, all_monkeys) {
                    path.push(name1.clone());
                    return Some(path);
                }

                if let Some(mut path) = Self::find_monkey_path(name2, monkey_name, all_monkeys) {
                    path.push(name2.clone());
                    return Some(path);
                }

                None
            }
            Self::Sub(name1, name2) => {
                if let Some(mut path) = Self::find_monkey_path(name1, monkey_name, all_monkeys) {
                    path.push(name1.clone());
                    return Some(path);
                }

                if let Some(mut path) = Self::find_monkey_path(name2, monkey_name, all_monkeys) {
                    path.push(name2.clone());
                    return Some(path);
                }

                None
            }
        }
    }

    fn resolve_value(
        &self,
        goal_value: i64,
        monkey_path: &Vec<String>,
        all_monkeys: &HashMap<String, Monkey>,
        mem: &mut HashMap<String, i64>,
    ) -> i64 {
        match self {
            Self::Num(_) => goal_value,
            Self::Add(m1, m2) => {
                if monkey_path.contains(m1) {
                    let left_hand_side = all_monkeys.get(m1).unwrap();
                    let right_hand_side_value = mem.get(m2).unwrap();
                    let new_goal = goal_value - right_hand_side_value;
                    return left_hand_side.resolve_value(new_goal, monkey_path, all_monkeys, mem);
                }

                let right_hand_side = all_monkeys.get(m2).unwrap();
                let left_hand_side_value = mem.get(m1).unwrap();
                let new_goal =  goal_value - left_hand_side_value;
                return right_hand_side.resolve_value(new_goal, monkey_path, all_monkeys, mem);
            },
            Self::Sub(m1, m2) => {
                if monkey_path.contains(m1) {
                    let left_hand_side = all_monkeys.get(m1).unwrap();
                    let right_hand_side_value = mem.get(m2).unwrap();
                    let new_goal = goal_value + right_hand_side_value;
                    return left_hand_side.resolve_value(new_goal, monkey_path, all_monkeys, mem);
                }

                let right_hand_side = all_monkeys.get(m2).unwrap();
                let left_hand_side_value = mem.get(m1).unwrap();
                let new_goal = left_hand_side_value - goal_value;
                return right_hand_side.resolve_value(new_goal, monkey_path, all_monkeys, mem);
            },
            Self::Mul(m1, m2) => {
                if monkey_path.contains(m1) {
                    let left_hand_side = all_monkeys.get(m1).unwrap();
                    let right_hand_side_value = mem.get(m2).unwrap();
                    let new_goal = goal_value / right_hand_side_value;
                    return left_hand_side.resolve_value(new_goal, monkey_path, all_monkeys, mem);
                }

                let right_hand_side = all_monkeys.get(m2).unwrap();
                let left_hand_side_value = mem.get(m1).unwrap();
                let new_goal = goal_value / left_hand_side_value;
                return right_hand_side.resolve_value(new_goal, monkey_path, all_monkeys, mem);
            },
            Self::Div(m1, m2) => {
                if monkey_path.contains(m1) {
                    let left_hand_side = all_monkeys.get(m1).unwrap();
                    let right_hand_side_value = mem.get(m2).unwrap();
                    let new_goal = goal_value * right_hand_side_value;
                    return left_hand_side.resolve_value(new_goal, monkey_path, all_monkeys, mem);
                }

                let right_hand_side = all_monkeys.get(m2).unwrap();
                let left_hand_side_value = mem.get(m1).unwrap();
                let new_goal = goal_value / left_hand_side_value;
                return right_hand_side.resolve_value(new_goal, monkey_path, all_monkeys, mem);
            },
        }
    }
}

fn parse_input() -> HashMap<String, Monkey> {
    let mut monkey_map = HashMap::default();

    fs::read_to_string("input.txt")
        .expect("Unable to read file!")
        .lines()
        .for_each(|line| {
            let (monkey_name, monkey_job) = line.split_once(": ").unwrap();
            if let Ok(monkey_number) = monkey_job.parse::<i64>() {
                monkey_map.insert(monkey_name.to_string(), Monkey::Num(monkey_number));
            } else if let Some((m1, m2)) = monkey_job.split_once(" + ") {
                monkey_map.insert(
                    monkey_name.to_string(),
                    Monkey::Add(m1.to_string(), m2.to_string()),
                );
            } else if let Some((m1, m2)) = monkey_job.split_once(" - ") {
                monkey_map.insert(
                    monkey_name.to_string(),
                    Monkey::Sub(m1.to_string(), m2.to_string()),
                );
            } else if let Some((m1, m2)) = monkey_job.split_once(" * ") {
                monkey_map.insert(
                    monkey_name.to_string(),
                    Monkey::Mul(m1.to_string(), m2.to_string()),
                );
            } else if let Some((m1, m2)) = monkey_job.split_once(" / ") {
                monkey_map.insert(
                    monkey_name.to_string(),
                    Monkey::Div(m1.to_string(), m2.to_string()),
                );
            }
        });

    monkey_map
}

fn part1() {
    let monkeys = parse_input();
    let mut mem = HashMap::default();
    let value = Monkey::get_value(&"root".to_string(), &monkeys, &mut mem);

    println!("Result: {value}");
}

fn part2() {
    let monkeys = parse_input();
    let mut mem = HashMap::default();
    Monkey::get_value(&"root".to_string(), &monkeys, &mut mem);

    let root_monkey = monkeys.get(&"root".to_string()).unwrap();
    let value = match root_monkey {
        Monkey::Add(m1, m2) => {
            if let Some(mut path) = Monkey::find_monkey_path(m1, &"humn".to_string(), &monkeys) {
                let goal_value = mem.get(m2).unwrap();
                path.push(m1.clone());

                let unknown_monkey = monkeys.get(m1).unwrap();
                unknown_monkey.resolve_value(*goal_value, &path, &monkeys, &mut mem)
            } else {
                -1
            }
        }
        _ => todo!("Monkey operator {:?} not supported", root_monkey),
    };
    println!("Result: {value}");
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
