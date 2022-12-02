use std::env;
use std::fs;
#[derive(Debug)]
enum Shape {
    Rock(i32),
    Paper(i32),
    Scissor(i32),
}

impl Shape {
    fn value(self: &Shape) -> i32 {
        match self {
            Self::Rock(_) => 1,
            Self::Paper(_) => 2,
            Self::Scissor(_) => 3,
        }
    }

    fn from_str(str: &str) -> Shape {
        if str == "A" {
            return Shape::Rock(1);
        }
        if str == "B" {
            return Shape::Paper(1);
        }
        if str == "C" {
            return Shape::Scissor(1);
        }

        if str == "X" {
            return Shape::Rock(2);
        }
        if str == "Y" {
            return Shape::Paper(2);
        }
        if str == "Z" {
            return Shape::Scissor(2);
        }

        todo!("Oh dear")
    }
}

enum Result {
    Loss,
    Win,
    Draw
}

impl Result {
    fn from_str(str: &str) -> Result {
        if str == "X" {
            return Result::Loss
        }
        if str == "Y" {
            return Result::Draw
        }
        if str == "Z" {
            return Result::Win
        }
        todo!("Shizzle")
    }

    fn value(self: &Result) -> i32 {
        match self {
            Self::Draw => 3,
            Self::Win => 6,
            Self::Loss => 0
        }
    }
}

#[derive(Debug)]
struct Round(Shape, Shape);

struct Outcome(Shape, Result);

impl Outcome {
    fn get_shape(self: &Outcome) -> Shape {
        match self.0 {
            Shape::Rock(_) => match self.1 {
                Result::Win => Shape::Paper(2),
                Result::Loss => Shape::Scissor(2),
                Result::Draw => Shape::Rock(2)
            }
            Shape::Paper(_) => match self.1 {
                Result::Win => Shape::Scissor(2),
                Result::Loss => Shape::Rock(2),
                Result::Draw => Shape::Paper(2)
            }
            Shape::Scissor(_) => match self.1 {
                Result::Win => Shape::Rock(2),
                Result::Loss => Shape::Paper(2),
                Result::Draw => Shape::Scissor(2)
            }
        }
    }
}


fn calc_result(round: &Round) -> i32 {
    let win = match round.1 {
        Shape::Rock(_) => {
            match round.0 {
                Shape::Scissor(_) => 6,
                Shape::Rock(_) => 3,
                _ => 0
            }
        }
        Shape::Paper(_) => {
            match round.0 {
                Shape::Rock(_) => 6,
                Shape::Paper(_) => 3,
                _ => 0
            }
        }
        Shape::Scissor(_) => {
            match round.0 {
                Shape::Paper(_) => 6,
                Shape::Scissor(_) => 3,
                _ => 0
            }
        }
    };
    println!("{:?}, win:{}, val:{}", round, win, round.1.value());
    win + round.1.value()
}

fn calc_outcome(outcome: &Outcome) -> i32 {
    outcome.get_shape().value() + outcome.1.value()
}

fn parse_input() -> String {
    fs::read_to_string("input.txt").expect("Unable to read file!")
}

fn get_rounds() -> Vec<Round> {
    let content = parse_input();
    let mut rounds: Vec<Round> = vec![];
    for line in content.lines() {
        let mut line_split = line.split_whitespace();
        let a = line_split.next().unwrap();
        let b = line_split.next().unwrap();

        rounds.push(Round(Shape::from_str(a), Shape::from_str(b)));
    }

    return rounds;
}

fn get_outcomes() -> Vec<Outcome> {
    let content = parse_input();
    let mut outcomes: Vec<Outcome> = vec![];
    for line in content.lines() {
        let mut line_split = line.split_whitespace();
        let a = line_split.next().unwrap();
        let b = line_split.next().unwrap();

        outcomes.push(Outcome(Shape::from_str(a), Result::from_str(b)));
    }

    return outcomes;
}

fn part1() {
    let rounds = get_rounds();
    let result = rounds.iter().fold(0,|sum, round| sum + calc_result(round));
    println!("Result: {}", result)
}

fn part2() {
    let outcomes = get_outcomes();
    let result = outcomes.iter().fold(0,|sum, outcome| sum + calc_outcome(outcome));
    println!("Result: {}", result)
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
