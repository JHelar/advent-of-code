use std::collections::HashMap;

const PADDING_COUNT: isize = 300;

fn rule_entry_from_str(str: &str) -> (String, String) {
    let (rule, result) = str.trim().split_once(" => ").unwrap();
    (rule.to_string(), result.to_string())
}

fn read_input() -> (String, HashMap<String, String>) {
    let mut initial_state_line = String::new();

    let _ = std::io::stdin().read_line(&mut initial_state_line);
    let initial_state = initial_state_line.trim().replace("initial state: ", "");

    let rules = std::io::stdin()
        .lines()
        .filter_map(|line| line.ok())
        .filter(|line| !line.is_empty())
        .map(|line| rule_entry_from_str(&line))
        .collect();

    let mut state = String::from("...");
    state.push_str(&initial_state);
    state.push_str("...");
    (state, rules)
}

fn count_plants(state: &String, generation: isize) -> isize {
    state
        .chars()
        .enumerate()
        .filter(|(_, plant)| plant == &'#')
        .map(|(i, _)| (i as isize - (3 + generation)))
        .sum()
}

fn step(state: &String, rules: &HashMap<String, String>) -> String {
    let mut next_state = String::from("...");
    for index in 2..state.len() - 2 {
        let llcrr = &state[index - 2..=index + 2].to_string();
        if let Some(plant) = rules.get(llcrr) {
            next_state.push_str(plant);
        } else {
            next_state.push_str(".")
        }
    }
    next_state.push_str("...");
    next_state
}

fn get_key(state: &String) -> String {
    let no_front: String = state.chars().skip_while(|pot| pot == &'.').collect();
    let no_back: String = no_front
        .chars()
        .rev()
        .skip_while(|pot| pot == &'.')
        .collect();

    no_back.chars().rev().collect()
}

fn part1() -> Option<u64> {
    let (initial_state, rules) = read_input();
    let mut state = initial_state;
    let generations: isize = 20;

    for _ in 1..=generations {
        state = step(&state, &rules);
    }

    let result = count_plants(&state, generations);
    Some(result as u64)
}

fn part2() -> Option<u64> {
    let (initial_state, rules) = read_input();
    let mut state = initial_state;
    let generations: isize = 50_000_000_000;

    let mut mem: HashMap<isize, isize> = HashMap::default();
    let mut prev_count = 0;
    let mut result = 0;

    for generation in 1..=generations {
        state = step(&state, &rules);
        let count = count_plants(&state, generation);
        let diff =  count - prev_count;
        let e = mem.entry(diff).or_insert(0);
        
        if *e > 10 {
            result = (generations - generation) * diff + count;
            break;
        } else {
            *e += 1;
            prev_count = count;
        }
    }
    Some(result as u64)
}

fn main() {
    println!("--- Day 12: Subterranean Sustainability ---");
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
