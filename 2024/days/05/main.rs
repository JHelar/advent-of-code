use std::collections::HashMap;

type Rules = HashMap<usize, Vec<usize>>;

#[derive(Debug, PartialEq, Eq)]
struct Manual(Vec<usize>);

impl Manual {
    fn from_str(line: &str) -> Self {
        Self(line.split(",").map(|page| page.parse().unwrap()).collect())
    }

    fn to_sorted(&self, rules: &Rules) -> Self {
        let mut pages = self.0.clone();
        pages.sort_by(|a, b| match (rules.get(a), rules.get(b)) {
            (Some(a_rule), Some(b_rule)) => {
                if a_rule.contains(b) {
                    std::cmp::Ordering::Greater
                } else if b_rule.contains(a) {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Equal
                }
            }
            (Some(a_rule), None) => {
                if a_rule.contains(b) {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Equal
                }
            }
            (None, Some(b_rule)) => {
                if b_rule.contains(a) {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Equal
                }
            }
            _ => {
                panic!("Should not happen")
            }
        });

        Self(pages)
    }

    fn value(&self) -> usize {
        self.0[(self.0.len() - 1) / 2]
    }
}

fn read_input() -> (Rules, Vec<Manual>) {
    let mut rules: Rules = HashMap::new();
    let mut read_rules = true;
    let mut pages = Vec::new();

    for line in std::io::stdin()
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| line.trim().to_string())
    {
        if line.is_empty() {
            read_rules = false
        } else if read_rules {
            let (x, y) = line.split_once("|").unwrap();

            let rule_entry = rules.entry(y.parse().unwrap()).or_insert(Vec::new());
            rule_entry.push(x.parse().unwrap());
        } else {
            pages.push(Manual::from_str(&line));
        }
    }

    (rules, pages)
}

fn part1() -> Option<usize> {
    let (rules, manuals) = read_input();
    let mut result = 0;

    for manual in manuals {
        let sorted_manual = manual.to_sorted(&rules);
        if manual == sorted_manual {
            result += manual.value();
        }
    }

    Some(result)
}

fn part2() -> Option<usize> {
    let (rules, manuals) = read_input();
    let mut result = 0;

    for manual in manuals {
        let sorted_manual = manual.to_sorted(&rules);
        if manual != sorted_manual {
            result += sorted_manual.value();
        }
    }

    Some(result)
}

fn main() {
    println!("--- Day 5: Print Queue ---");
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
