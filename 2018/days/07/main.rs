use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

#[derive(Debug)]
struct Step {
    name: char,
    children: BTreeSet<char>,
    parents: BTreeSet<char>,
}

impl Step {
    fn from_instructions(char: char, instructions: &Vec<Instruction>) -> Self {
        let children: BTreeSet<char> = instructions
            .iter()
            .filter_map(|instruction| {
                if instruction.0 == char {
                    Some(instruction.1)
                } else {
                    None
                }
            })
            .collect();

        let parents: BTreeSet<char> = instructions
            .iter()
            .filter_map(|instruction| {
                if instruction.1 == char {
                    Some(instruction.0)
                } else {
                    None
                }
            })
            .collect();

        Self {
            name: char,
            children,
            parents,
        }
    }

    fn can_visit(&self, visited: &Vec<char>) -> bool {
        if self.parents.len() == 0 {
            true
        } else {
            self.parents
                .iter()
                .all(|parent_name| visited.contains(parent_name))
        }
    }
}

#[derive(Debug)]
struct Instruction(char, char);

impl Instruction {
    fn from_str(str: &str) -> Self {
        let (from_str, to_str) = str.split_once(" must be finished before step ").unwrap();

        let from = from_str.chars().last().unwrap();
        let to = to_str.chars().next().unwrap();

        Self(from, to)
    }
}

fn read_input() -> Vec<Instruction> {
    std::io::stdin()
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| Instruction::from_str(line.trim()))
        .collect()
}

fn process_instructions(instructions: &Vec<Instruction>, max_workers: usize, step_duration: u32) -> (String, u32) {
    let unique_steps: HashSet<char> = instructions
        .iter()
        .flat_map(|instruction| vec![instruction.0, instruction.1])
        .collect();

    let steps: HashMap<char, Step> = unique_steps
        .iter()
        .map(|char| (*char, Step::from_instructions(*char, &instructions)))
        .collect();

    let mut visit = steps
        .iter()
        .filter_map(|(_, step)| {
            if step.parents.len() == 0 {
                Some(step.name)
            } else {
                None
            }
        })
        .collect::<BTreeSet<char>>();

    let mut completed = Vec::default();
    let mut workers: Vec<Option<(&Step, u32, u32)>> = vec![None;max_workers];
    let mut elapsed_time = 0;

    while completed.len() != steps.len() {
        // If we have workers available and we have things to visit
        for worker_index in 0..max_workers {
            if workers[worker_index].is_some() {
                continue;
            } else if let Some(step_name) = visit.pop_first() {
                let work_time = 1 + (step_name as u8 - 65) as u32 + step_duration;
                workers[worker_index] = Some((steps.get(&step_name).unwrap(), 0, work_time));
            }
        }
        // Process workers
        for worker_index in 0..max_workers {
            if let Some(worker) = workers.get_mut(worker_index).unwrap() {
                worker.1 += 1;
                if worker.1 == worker.2 {
                    completed.push(worker.0.name);
                    worker.0.children.iter().for_each(|child_name| {
                        if !visit.contains(child_name)
                            && steps.get(child_name).unwrap().can_visit(&completed)
                        {
                            visit.insert(*child_name);
                        }
                    });
                    workers[worker_index] = None;
                }
            }
        }
        elapsed_time += 1
    }

    (completed.iter().collect(), elapsed_time)
}

fn part1() -> Option<String> {
    let instructions = read_input();
    let (steps, _) = process_instructions(&instructions, 1, 0);
    Some(steps)
}

fn part2() -> Option<String> {
    let instructions = read_input();
    let (_, time) = process_instructions(&instructions, 5, 60);
    Some(format!("{time}"))
}

fn main() {
    println!("--- Day 7: The Sum of Its Parts ---");
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
