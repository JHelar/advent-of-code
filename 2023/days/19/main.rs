use std::collections::HashMap;
use rayon::prelude::*;

#[derive(Debug, PartialEq, Eq, Hash)]
enum Part {
  X,
  M,
  A,
  S
}

impl Part {
    fn from_string(str: &str) -> Self {
      match str {
          "x" => Self::X,
          "m" => Self::M,
          "a" => Self::A,
          "s" => Self::S,
          _ => panic!("Unknown part {str}")
      }
    }
}

#[derive(Debug)]
enum Rule {
  LT { part: Part, count: u32, to_workflow: String },
  GT { part: Part, count: u32, to_workflow: String },
  Else { to_workflow: String }
}

impl Rule {
  fn from_string(str: &str) -> Self {
    if let Some((part_str, count_label_str)) = str.split_once('>') {
        let (count_str, to_workflow) = count_label_str.split_once(':').unwrap();
        Self::GT { part: Part::from_string(part_str), count: count_str.parse().unwrap(), to_workflow: to_workflow.to_string() }
    } else if let Some((part_str, count_label_str)) = str.split_once('<')  {
      let (count_str, to_workflow) = count_label_str.split_once(':').unwrap();
      Self::LT { part: Part::from_string(part_str), count: count_str.parse().unwrap(), to_workflow: to_workflow.to_string() }
    } else {
      Self::Else { to_workflow: str.to_string() }
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
      let ( workflow_name, rules ) = str.clone().split_once('{').unwrap();
      let rules = rules[0..rules.len() - 1].split(',').map(Rule::from_string).collect();
      Self {
        rules,
        name: workflow_name.to_string()
      }
    }

    fn process_part_rating(&self, part_rating: &HashMap<Part, u32>) -> String {
      for rule in self.rules.iter() {
          match rule {
              Rule::GT { part, count, to_workflow } => {
                let part_count = part_rating.get(part).unwrap();
                if part_count > count {
                  return to_workflow.clone();
                }
              },
              Rule::LT { part, count, to_workflow } => {
                let part_count = part_rating.get(part).unwrap();
                if part_count < count {
                  return to_workflow.clone();
                }
              },
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
          line[1..line.len()-1].split(",").for_each(|part_rating_str| {
            let (part_str, count_str) = part_rating_str.split_once("=").unwrap();
            part_rating.insert(Part::from_string(part_str), count_str.parse().unwrap());
          });
          part_ratings.push(part_rating);
        }
      });
  (workflows, part_ratings)
}

fn process_part_rating(workflow_name: String, part_rating: &HashMap<Part, u32>, workflows: &HashMap<String, Workflow>) -> String {
    if let Some(workflow) = workflows.get(&workflow_name) {
      let next_workflow = workflow.process_part_rating(part_rating);
      return process_part_rating(next_workflow, part_rating, workflows);
    } else {
      return workflow_name;
    }
}

fn sum_part_rating(part_rating: &HashMap<Part, u32>) -> u64 {
  let res: u32 = part_rating.iter().map(|(_, count)| count).sum();
  res as u64
}

fn calculate_total_combinations() -> u64 {
  4000u64 * 4000 * 4000 * 4000
}

fn generate_combination(index: u64) -> HashMap<Part, u32> {
  let mut combination = HashMap::new();

  combination.insert(Part::X, 1 + (index % 4000) as u32);
  combination.insert(Part::M, 1 + ((index / 4000) % 4000) as u32);
  combination.insert(Part::A, 1 + ((index / (4000 * 4000)) % 4000) as u32);
  combination.insert(Part::S, 1 + ((index / (4000 * 4000 * 4000)) % 4000) as u32);

  combination
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

    let total_combinations = calculate_total_combinations(); // Replace with your logic to get the total number of combinations
    let result = (0..total_combinations).into_iter()
        .par_bridge()
        .filter(|index| {
            let part_rating = generate_combination(*index);
            let workflow_name = "in".to_string();
            let final_workflow = process_part_rating(workflow_name, &part_rating, &workflows);
            final_workflow == "A".to_string()
        }).count() as u64;

    Some(result)
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
