use std::fs;
use std::collections::HashSet;

enum Number {
    Positive(i32),
    Negative(i32)
}

fn parse_input() -> Vec<Number> {
    let contents = fs::read_to_string("./day1.input").expect("Unable to read file!");
    let lines = contents.split("\n");
    let mut numbers:Vec<Number> = vec![];
    for line in lines {
        let sign = &line[0..1];
        let value = &line[1..];
        let numeric_value = i32::from_str_radix(&value, 10).unwrap();

        if sign == "+" {
            numbers.push(Number::Positive(numeric_value));
        } else {
            numbers.push(Number::Negative(numeric_value));
        }
    }

    return numbers;
}

pub fn part1() {
    let numbers = parse_input();
    let mut result: i32 = 0;
    for number in numbers {
        match number {
            Number::Positive(value) => {
                result += value;
            },
            Number::Negative(value) => {
                result -= value;
            }
        }
    }

    println!("Result: {}", result);
}

pub fn part2() {
    let numbers = parse_input();
    let mut results: HashSet<i32> = HashSet::new();
    let mut result: i32 = 0;
    results.insert(result);

    loop {
        for number in &numbers {
            match number {
                Number::Positive(value) => {
                    result += value;
                },
                Number::Negative(value) => {
                    result -= value;
                }
            }

            if results.contains(&result) {
                println!("Found it: {}", result);
                return
            }
            results.insert(result.clone());
        }
    }
}