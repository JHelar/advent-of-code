use std::env;
use std::fs;

fn parse_input() -> Vec<i32> {
    let contents = fs::read_to_string("input.txt").expect("Unable to read file!");
    let mut elfs: Vec<i32> = vec![];
    let mut sum: i32 = 0;

    for line in contents.split('\n') {
        if line == "" {
            elfs.push(sum.clone());
            sum = 0;
        } else {
            let value = line.parse::<i32>().unwrap();
            sum += value;
        }
    }

    return elfs;
}

fn part1() {
    let elfs = parse_input();
    let mut max = 0;
    for elf in elfs {
        if elf > max {
            max = elf;
        }
    }

    println!("The best elf is: {}", max);
}

fn part2() {
    let mut elfs = parse_input();
    elfs.sort_by(|a, b| b.partial_cmp(a).unwrap());

    let mut sum = 0;
    for elf in &elfs[..=2] {
        sum += elf;
    }

    println!("Top three elf sum: {}", sum);
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
