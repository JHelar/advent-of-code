use rayon::prelude::*;
use std::{collections::HashMap, fmt::Display};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Part {
    X,
    M,
    A,
    S,
}

impl Part {
    fn from_string(str: &str) -> Self {
        match str {
            "x" => Self::X,
            "m" => Self::M,
            "a" => Self::A,
            "s" => Self::S,
            _ => panic!("Unknown part {str}"),
        }
    }
}

impl Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Self::X => 'x',
            Self::M => 'm',
            Self::A => 'a',
            Self::S => 's',
        };
        write!(f, "{c}")
    }
}

#[derive(Debug)]
enum Rule {
    LT {
        part: Part,
        count: u32,
        to_workflow: String,
    },
    GT {
        part: Part,
        count: u32,
        to_workflow: String,
    },
    Else {
        to_workflow: String,
    },
}

impl Rule {
    fn from_string(str: &str) -> Self {
        if let Some((part_str, count_label_str)) = str.split_once('>') {
            let (count_str, to_workflow) = count_label_str.split_once(':').unwrap();
            Self::GT {
                part: Part::from_string(part_str),
                count: count_str.parse().unwrap(),
                to_workflow: to_workflow.to_string(),
            }
        } else if let Some((part_str, count_label_str)) = str.split_once('<') {
            let (count_str, to_workflow) = count_label_str.split_once(':').unwrap();
            Self::LT {
                part: Part::from_string(part_str),
                count: count_str.parse().unwrap(),
                to_workflow: to_workflow.to_string(),
            }
        } else {
            Self::Else {
                to_workflow: str.to_string(),
            }
        }
    }
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl Workflow {
    fn from_string(str: &str) -> Self {
        let (workflow_name, rules) = str.clone().split_once('{').unwrap();
        let rules = rules[0..rules.len() - 1]
            .split(',')
            .map(Rule::from_string)
            .collect();
        Self {
            rules,
            name: workflow_name.to_string(),
        }
    }

    fn process_part_rating(&self, part_rating: &HashMap<Part, u32>) -> String {
        for rule in self.rules.iter() {
            match rule {
                Rule::GT {
                    part,
                    count,
                    to_workflow,
                } => {
                    let part_count = part_rating.get(part).unwrap();
                    if part_count > count {
                        return to_workflow.clone();
                    }
                }
                Rule::LT {
                    part,
                    count,
                    to_workflow,
                } => {
                    let part_count = part_rating.get(part).unwrap();
                    if part_count < count {
                        return to_workflow.clone();
                    }
                }
                Rule::Else { to_workflow } => {
                    return to_workflow.clone();
                }
            }
        }
        panic!("Unable to process!")
    }
}

fn read_input() -> (HashMap<String, Workflow>, Vec<HashMap<Part, u32>>) {
    let mut parsing_workflows = true;
    let mut workflows = HashMap::new();
    let mut part_ratings = Vec::new();

    std::io::stdin()
        .lines()
        .filter_map(|line| line.ok())
        .for_each(|line| {
            if line.trim().is_empty() {
                parsing_workflows = false;
            } else if parsing_workflows {
                let workflow = Workflow::from_string(line.trim());
                workflows.insert(workflow.name.clone(), workflow);
            } else {
                let mut part_rating = HashMap::new();
                line[1..line.len() - 1]
                    .split(",")
                    .for_each(|part_rating_str| {
                        let (part_str, count_str) = part_rating_str.split_once("=").unwrap();
                        part_rating.insert(Part::from_string(part_str), count_str.parse().unwrap());
                    });
                part_ratings.push(part_rating);
            }
        });
    (workflows, part_ratings)
}

fn process_part_rating(
    workflow_name: String,
    part_rating: &HashMap<Part, u32>,
    workflows: &HashMap<String, Workflow>,
) -> String {
    if let Some(workflow) = workflows.get(&workflow_name) {
        let next_workflow = workflow.process_part_rating(part_rating);
        return process_part_rating(next_workflow, part_rating, workflows);
    } else {
        return workflow_name;
    }
}

fn count_combinations(ranges: &HashMap<Part, (u32, u32)>) -> u64 {
    let total_combinations = ranges
        .values()
        .fold(1, |acc, &(start, end)| acc * ((end - start) + 1) as u64);
    total_combinations as u64
}

fn get_ranges(
    workflow_name: String,
    part_ranges: &HashMap<Part, (u32, u32)>,
    workflows: &HashMap<String, Workflow>,
) -> Vec<HashMap<Part, (u32, u32)>> {
    let mut ranges: Vec<HashMap<Part, (u32, u32)>> = Vec::new();
    let mut else_ranges = part_ranges.clone();

    if let Some(workflow) = workflows.get(&workflow_name) {
        workflow.rules.iter().for_each(|rule| {
            match rule {
                Rule::LT {
                    part,
                    count,
                    to_workflow,
                } => {
                    let mut ranges_copy = else_ranges.clone();
                    ranges_copy.get_mut(&part).unwrap().1 =
                        ranges_copy.get_mut(&part).unwrap().1.min(count - 1);
                    else_ranges.get_mut(&part).unwrap().0 =
                        else_ranges.get_mut(&part).unwrap().0.max(*count);
                    if to_workflow == "A" {
                        ranges.push(ranges_copy);
                    } else if to_workflow != "R" {
                        let child_ranges = get_ranges(to_workflow.clone(), &ranges_copy, workflows);
                        child_ranges
                            .into_iter()
                            .for_each(|range| ranges.push(range));
                    }
                }
                Rule::GT {
                    part,
                    count,
                    to_workflow,
                } => {
                    let mut ranges_copy = else_ranges.clone();
                    ranges_copy.get_mut(&part).unwrap().0 =
                        ranges_copy.get_mut(&part).unwrap().0.max(count + 1);
                    else_ranges.get_mut(&part).unwrap().1 =
                        else_ranges.get_mut(&part).unwrap().1.min(*count);
                    if to_workflow == "A" {
                        ranges.push(ranges_copy);
                    } else if to_workflow != "R" {
                        let child_ranges = get_ranges(to_workflow.clone(), &ranges_copy, workflows);
                        child_ranges
                            .into_iter()
                            .for_each(|range| ranges.push(range));
                    }
                }
                Rule::Else { to_workflow } => {
                    if to_workflow == "A" {
                        ranges.push(else_ranges.clone());
                    } else if to_workflow != "R" {
                        let child_ranges = get_ranges(to_workflow.clone(), &else_ranges, workflows);
                        child_ranges
                            .into_iter()
                            .for_each(|range| ranges.push(range));
                    }
                }
            };
        })
    }
    ranges
}

fn sum_part_rating(part_rating: &HashMap<Part, u32>) -> u64 {
    let res: u32 = part_rating.iter().map(|(_, count)| count).sum();
    res as u64
}

fn part1() -> Option<u64> {
    let (workflows, part_ratings) = read_input();

    let mut result = 0_u64;
    for part_rating in part_ratings.iter() {
        let res = process_part_rating("in".to_string(), part_rating, &workflows);
        if res == "A".to_string() {
            result += sum_part_rating(part_rating);
        }
    }
    Some(result)
}

fn part2() -> Option<u64> {
    let (workflows, _) = read_input();
    let part_ranges = [
        (Part::X, (1_u32, 4000_u32)),
        (Part::M, (1_u32, 4000_u32)),
        (Part::A, (1_u32, 4000_u32)),
        (Part::S, (1_u32, 4000_u32)),
    ]
    .into_iter()
    .collect();

    let ranges = get_ranges("in".to_string(), &part_ranges, &workflows);
    let sums = ranges
        .iter()
        .map(|range| count_combinations(range))
        .collect::<Vec<u64>>();
    let sum: u64 = sums.iter().sum();
    Some(sum)
}

fn main() {
    println!("--- Day 19: Aplenty ---");
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
