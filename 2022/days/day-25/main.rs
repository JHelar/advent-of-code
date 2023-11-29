use std::env;
use std::fs;

struct SNAFU {}

impl SNAFU {
    fn decode(snafu_str: &str) -> i64 {
        let iter = snafu_str.chars().map(Self::char_to_dec).rev().enumerate();
        let mut result = 0;
        for (exp, num) in iter {
            result += num * 5_i64.pow(exp as u32)
        }
        result
    }

    fn encode(decimal: i64) -> String {
        let mut snafu = String::new();

        let mut current = decimal;
        let max_exp = (0..).find(|exp| 5_i64.pow(*exp as u32) >= current).unwrap();

        for exp in (0..max_exp).rev() {
            let base = 5_i64.pow(exp as u32);
            let count = (current as f64 / base as f64).round() as i64;
            let str = Self::num_to_str(count);
            
            current -= count * base;
            snafu.push_str(&str);
        }

        snafu
    }

    fn num_to_str(num: i64) -> String {
        match num {
            -2 => "=",
            -1 => "-",
            0 => "0",
            1 => "1",
            2 => "2",
            _ => panic!("Unable to convert {num}"),
        }.to_string()
    }

    fn char_to_dec(c: char) -> i64 {
        match c {
            '-' => -1,
            '=' => -2,
            _ => c.to_string().parse().unwrap(),
        }
    }
}

fn parse_input() -> Vec<String> {
    fs::read_to_string("input.txt")
        .expect("Unable to read file!")
        .lines()
        .map(|line| line.trim().to_string())
        .collect()
}

fn part1() {
    let numbers = parse_input();

    let mut sum = 0;
    for snafu in numbers {
        let decoded = SNAFU::decode(&snafu);
        sum += decoded;
    }
    
    let encoded = SNAFU::encode(sum);
    println!("Result: {encoded}");
}

fn part2() {}

fn main() {
    let args: Vec<String> = env::args().collect();
    let part = &args[1];

    if part == "1" {
        part1();
    } else if part == "2" {
        part2();
    }
}
