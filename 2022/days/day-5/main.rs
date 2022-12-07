use std::collections::VecDeque;
use std::env;
use std::fs;

const STACKS_SIZE: usize = 9;

type Stack = VecDeque<String>;

struct Stacks([Stack; STACKS_SIZE]);

impl Stacks {
    fn follow_direction(self: &mut Stacks, direction: &Direction) {
        for _i in 0..direction.0 {
            if let Some(elem) = &self.0[direction.1].pop_back() {
                self.0[direction.2].push_back(elem.to_string());
            }
        }
    }

    fn follow_direction_9001(self: &mut Stacks, direction: &Direction) {
        let mut tmp: VecDeque<String> = VecDeque::new();

        for _i in 0..direction.0 {
            if let Some(elem) = &self.0[direction.1].pop_back() {
                tmp.push_back(elem.to_string());
            }
        }

        for _i in 0..direction.0 {
            if let Some(elem) = tmp.pop_back() {
                self.0[direction.2].push_back(elem);
            }
        }
    }



    fn print_ends(self: &Stacks) {
        for i in 0..STACKS_SIZE {
            if let Some(elem) = self.0[i].back() {
                print!("{}", elem);
            }
        }
    }
}

#[derive(Debug)]
struct Direction(i32, usize, usize);

fn parse_input() -> String {
    fs::read_to_string("input.txt").expect("Unable to read file!")
}

fn parse_instructions() -> (Stacks, Vec<Direction>) {
    let content = parse_input();
    let mut content_split = content.split("\n\n");
    let stacks_str = content_split.next().unwrap();
    let intruction_str = content_split.next().unwrap();

    let mut stacks: [Stack; STACKS_SIZE] = Default::default();
    for line in stacks_str.lines() {
        let mut index = 0;

        for stack_str in line
            .split("    ")
            .flat_map(|x| x.split(' ').map(|y| y.trim()))
        {
            if stack_str != "" && stack_str.contains('[') {
                stacks[index].push_front(stack_str[1..][..1].to_string());
            }
            index += 1;
        }
    }

    let mut directions: Vec<Direction> = vec![];
    for line in intruction_str.lines() {
        let mut instruction_data = line.split(' ').filter(|x| x.parse::<i32>().is_ok());

        directions.push(Direction(
            instruction_data.next().unwrap().parse::<i32>().unwrap(),
            instruction_data.next().unwrap().parse::<usize>().unwrap() - 1,
            instruction_data.next().unwrap().parse::<usize>().unwrap() - 1,
        ));
    }

    (Stacks(stacks), directions)
}

fn part1() {
    let mut instructions = parse_instructions();
    for direction in instructions.1 {
        instructions.0.follow_direction(&direction);
    }

    instructions.0.print_ends()
}

fn part2() {
    let mut instructions = parse_instructions();
    for direction in instructions.1 {
        instructions.0.follow_direction_9001(&direction);
    }

    instructions.0.print_ends()
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
