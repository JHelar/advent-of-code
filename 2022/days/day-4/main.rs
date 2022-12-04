use std::env;
use std::fs;

#[derive(Debug)]
struct Section(i32, i32);

impl Section {
    fn from_str(str: &str) -> Section {
        let mut splitted = str.split('-');
        let p1 = splitted.next().unwrap();
        let p2 = splitted.next().unwrap();

        Section(p1.parse::<i32>().unwrap(), p2.parse::<i32>().unwrap())
    }

    fn overlaps(self: &Section, other: &Section) -> bool {
        self.0 <= other.0 && self.1 >= other.1
    }

    fn partial_overlaps(self: &Section, other: &Section) -> bool {
        self.0 <= other.0 && self.1 >= other.0 || self.0 <= other.1 && self.1 >= other.1
    }
}

#[derive(Debug)]
struct Pair(Section, Section);

impl Pair {
    fn is_overlaped(self: &Pair) -> bool {
        self.0.overlaps(&self.1) || self.1.overlaps(&self.0) 
    }

    fn is_partially_overlaped(self: &Pair) -> bool {
        self.0.partial_overlaps(&self.1) || self.1.partial_overlaps(&self.0) 
    }

}

fn parse_input() -> String {
    fs::read_to_string("input.txt").expect("Unable to read file!")
}

fn parse_pairs() -> Vec<Pair> {
    parse_input().lines().map(|x| {
        let mut sections = x.split(',');
        let s1_str = sections.next().unwrap();
        let s2_str = sections.next().unwrap();

        Pair(Section::from_str(s1_str), Section::from_str(s2_str))
    }).collect()
}

fn part1() {
    let pairs = parse_pairs();
    let filtered_pairs = pairs.iter().filter(|x| x.is_overlaped()).collect::<Vec<&Pair>>();
    println!("Result: {} overlaps", filtered_pairs.len())
}

fn part2() {
    let pairs = parse_pairs();
    let filtered_pairs = pairs.iter().filter(|x| x.is_partially_overlaped()).collect::<Vec<&Pair>>();
    println!("Result: {} overlaps", filtered_pairs.len())
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
