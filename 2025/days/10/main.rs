use std::collections::VecDeque;

use hashbrown::{HashMap, HashSet};
use z3::{ast::Ast, ast::Int, Config, Context, Optimize, Solver};

type Button = Vec<usize>;
type Joltage = Vec<u16>;
type Lights = Vec<bool>;

#[derive(Debug)]
struct Machine {
    lights: Lights,
    buttons: Vec<Button>,
    joltage: Joltage,
    joltage_value: u16,
}

impl Machine {
    fn from_str(str: &str) -> Self {
        let mut parts = str.split_whitespace();
        let lights_str = parts.next().expect("Should have lights");
        let lights: Vec<bool> = lights_str[1..(lights_str.len() - 1)]
            .chars()
            .map(|c| match c {
                '.' => false,
                '#' => true,
                _ => panic!("Unexpected light token({c})"),
            })
            .collect();

        let mut buttons = Vec::default();
        let mut joltage = Vec::default();
        let mut base_joltage = u16::MAX;
        for button_or_joltage_str in parts {
            match &button_or_joltage_str[0..1] {
                "(" => buttons.push(
                    button_or_joltage_str[1..(button_or_joltage_str.len() - 1)]
                        .split(",")
                        .map(|c| c.parse().unwrap())
                        .collect(),
                ),
                "{" => {
                    joltage = button_or_joltage_str[1..(button_or_joltage_str.len() - 1)]
                        .split(",")
                        .map(|c| {
                            let joltage = c.parse::<u16>().unwrap();
                            if base_joltage > joltage {
                                base_joltage = joltage;
                            }
                            joltage
                        })
                        .collect()
                }
                c => panic!("Unexpected token '{c}'"),
            }
        }

        // joltage = joltage.into_iter().map(|j| (j - base_joltage) + 1).collect();
        let joltage_value = joltage_value(&joltage);
        Self {
            lights,
            buttons,
            joltage,
            joltage_value,
        }
    }

    fn get_fresh_light_state(&self) -> Lights {
        self.lights.iter().map(|_| false).collect()
    }

    fn completed_lights(&self, state: &Lights) -> bool {
        self.lights
            .iter()
            .enumerate()
            .all(|(index, &light)| light == state[index])
    }
}

fn press_lights_button(state: &mut Lights, button: &Button) {
    for light_index in button {
        state[*light_index] = !state[*light_index]
    }
}

fn joltage_value(joltage: &Joltage) -> u16 {
    joltage.iter().sum()
}

fn step_machine_lights(machine: &Machine) -> u32 {
    let mut button_queue = VecDeque::default();
    let mut cache: HashMap<Lights, u32> = HashMap::default();

    for button_index in 0..machine.buttons.len() {
        button_queue.push_front((button_index, machine.get_fresh_light_state(), 0));
    }

    let mut best_presses = u32::MAX;
    while let Some((button_index, mut state, presses)) = button_queue.pop_back() {
        if cache
            .get(&state)
            .is_some_and(|&previous_presses| previous_presses <= presses)
        {
            continue;
        } else {
            cache.insert(state.clone(), presses);
        }

        press_lights_button(&mut state, &machine.buttons[button_index]);
        let completed = machine.completed_lights(&state);
        let new_presses = presses + 1;

        if new_presses >= best_presses {
            continue;
        }

        if completed {
            if best_presses > new_presses {
                best_presses = new_presses
            }
            continue;
        }

        for next_button_index in 0..machine.buttons.len() {
            if next_button_index == button_index {
                continue;
            }
            let new_state = state.clone();
            button_queue.push_front((next_button_index, new_state, new_presses));
        }
    }

    best_presses
}

fn step_machine_jolts(machine: &Machine) -> u32 {
    // Go through jolt and create the equation to solve
    // b1 + b2 + ... = j1
    // b1 + b2 + ... = j2
    // b1 + b2 + ... = j3
    // b1 + b2 + ... = j4
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let optimizer = Optimize::new(&ctx);

    let button_variables: Vec<(Int, Button)> = machine
        .buttons
        .iter()
        .enumerate()
        .map(|(button_index, joltages)| {
            (
                Int::new_const(&ctx, format!("b{button_index}")),
                joltages.clone(),
            )
        })
        .collect();

    for (joltage_index, joltage) in machine.joltage.iter().enumerate() {
        let jolt_constant = Int::from_u64(&ctx, *joltage as u64);
        let jolt_buttons = Int::add(
            &ctx,
            &button_variables
                .iter()
                .filter_map(|(variable, joltages)| {
                    if joltages.contains(&joltage_index) {
                        Some(variable)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>(),
        );

        optimizer.assert(&jolt_buttons._eq(&jolt_constant));
    }

    for (button_var, _) in &button_variables {
        optimizer.assert(&button_var.ge(&Int::from_i64(&ctx, 0)));
    }

    let all_button_variables: Vec<&Int> = button_variables.iter().map(|(v, _)| v).collect();
    let total_sum = Int::add(&ctx, &all_button_variables);

    optimizer.minimize(&total_sum);
    match optimizer.check(&[]) {
        z3::SatResult::Sat => {
            let model = optimizer.get_model().unwrap();
            model.eval(&total_sum, true).unwrap().as_u64().unwrap() as u32
        }
        _ => panic!("Unsatisfiable"),
    }
}

fn read_input() -> Vec<Machine> {
    std::io::stdin()
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| Machine::from_str(&line.trim()))
        .collect()
}

fn part1() -> Option<u32> {
    let machines = read_input();
    let mut sum = 0;
    for machine in machines.iter() {
        let presses = step_machine_lights(&machine);
        println!("{presses}");
        sum += presses
    }
    Some(sum)
}

fn part2() -> Option<u32> {
    let machines = read_input();
    let mut sum = 0;
    for machine in machines.iter() {
        let presses = step_machine_jolts(&machine);
        sum += presses
    }
    Some(sum)
}

fn main() {
    println!("--- Day 10: Factory ---");
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
