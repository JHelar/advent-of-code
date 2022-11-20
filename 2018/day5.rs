use std::{fs, iter::Peekable, slice::Iter};

fn parse_input() -> String {
    fs::read_to_string("./day5.input").expect("Unable to read file!")
}

fn is_pair(a_char: &u8, b_char: &u8) -> bool {
    let res = a_char.abs_diff(*b_char);

    res == 32u8
}

fn react(mut chars: Peekable<Iter<u8>>) -> (bool, Vec<u8>) {
    let mut remainder: Vec<u8> = vec![];
    let mut did_react = false;

    loop {
        if let Some(a_char) = chars.next() {
            if let Some(b_char) = chars.peek() {
                if is_pair(a_char, *b_char) {
                    chars.next();
                    did_react = true;
                } else {
                    remainder.push(*a_char);
                }
            } else {
                remainder.push(*a_char);
            }
        } else {
            break;
        }
    }

    (did_react, remainder)
}

pub fn part1() {
    let input = parse_input();
    let mut chars = input.chars().map(|x| x as u8).collect::<Vec<u8>>();

    loop {
        let (did_react, remainder) = react(chars.iter().peekable());

        if !did_react {
            println!("Result: {}", remainder.len());
            return;
        }

        chars = remainder;
    }
}

pub fn part2() {
    let input = parse_input().chars().map(|x| x as u8).collect::<Vec<u8>>();
    let mut best_result = 0;
    let first_unit = 'A' as u8;
    let last_unit = 'Z' as u8;

    for unit_type in first_unit..last_unit + 1 {
        let mut chars = input
            .iter()
            .cloned()
            .filter(|x| *x != unit_type && *x != (unit_type + 32u8))
            .collect::<Vec<u8>>();

        loop {
            let (did_react, remainder) = react(chars.iter().peekable());

            if !did_react {
                let result = remainder.len();
                if best_result == 0 || best_result > result {
                    best_result = result;
                }
                break;
            }

            chars = remainder;
        }
    }

    println!("Result: {}", best_result)
}
