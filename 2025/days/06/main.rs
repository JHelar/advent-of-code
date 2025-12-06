#[derive(Debug, Clone)]
enum Operator {
    Multiply,
    Add,
}

impl Operator {
    fn from_str(str: &str) -> Option<Self> {
        match str.trim() {
            "*" => Some(Self::Multiply),
            "+" => Some(Self::Add),
            _ => None,
        }
    }

    fn solve(&self, a: u64, b: u64) -> u64 {
        match self {
            Self::Add => a + b,
            Self::Multiply => a * b,
        }
    }
}

#[derive(Debug)]
struct Problem {
    digits: Vec<String>,
    operator: Option<Operator>,
    max_size: usize,
}

impl Problem {
    fn new() -> Self {
        Self {
            digits: Vec::default(),
            operator: None,
            max_size: 0,
        }
    }

    fn calc_max_size(&mut self, op_or_digit: &str) -> bool {
        if op_or_digit.is_empty() {
            false
        } else if op_or_digit.parse::<u64>().is_ok() {
            self.max_size = self.max_size.max(op_or_digit.len());
            true
        } else {
            true
        }
    }

    fn add(&mut self, op_or_digit: &str) {
        if op_or_digit.trim().parse::<u64>().is_ok() {
            self.digits.push(op_or_digit.to_string());
        } else {
            self.operator = Operator::from_str(op_or_digit);
        }
    }

    fn solve(&self, as_column: bool) -> u64 {
        let operator = self.operator.clone().expect("Should have an operator");

        let mut digits = self.digits.clone();

        if as_column {
            let mut new_digits = Vec::default();
            for digit_index in 0..self.max_size {
                let mut accumulated_digit = String::new();
                for digit in digits.iter() {
                    let digit_str = &digit[digit_index..=digit_index];
                    if !digit_str.is_empty() {
                        accumulated_digit = format!("{accumulated_digit}{digit_str}");
                    }
                }
                new_digits.push(accumulated_digit);
            }
            digits = new_digits;
        }
        digits
            .into_iter()
            .map(|digit| digit.trim().parse::<u64>().unwrap())
            .reduce(|a, b| operator.solve(a, b))
            .unwrap()
    }
}

fn read_input() -> Vec<Problem> {
    let mut problems = Vec::default();
    let mut lines = Vec::default();
    for line in std::io::stdin().lines().filter_map(|line| line.ok()) {
        let mut column = 0;
        lines.push(line.clone());
        for op_or_digit in line.split(" ") {
            if (column + 1) > problems.len() {
                problems.push(Problem::new());
            }

            if problems[column].calc_max_size(op_or_digit) {
                column += 1;
            }
        }
    }

    problems.pop();

    for line in lines {
        let mut line_start = 0;
        for problem in problems.iter_mut() {
            let op_or_digit = &line[line_start..(line_start + problem.max_size)];
            line_start += problem.max_size + 1;
            problem.add(op_or_digit);
        }
    }

    problems
}

fn part1() -> Option<u64> {
    let problems = read_input();

    let sum = problems
        .into_iter()
        .map(|problem| problem.solve(false))
        .sum();

    Some(sum)
}

fn part2() -> Option<u64> {
    let problems = read_input();
    
    let sum = problems
        .into_iter()
        .map(|problem| problem.solve(true))
        .sum();

    Some(sum)
}

fn main() {
    println!("--- Day 6: Trash Compactor ---");
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
