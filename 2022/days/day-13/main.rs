use std::env;
use std::fs;

fn parse_input() -> String {
    fs::read_to_string("input.txt").expect("Unable to read file!")
}

fn part1() {

}

fn part2() {

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
