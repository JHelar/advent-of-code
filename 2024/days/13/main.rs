use std::collections::HashMap;

use vector2::Vector2;

mod vector2;

#[derive(Debug)]
struct ClawMachine {
    a: Vector2,
    b: Vector2,
    prize: Vector2,
}

fn get_cost_vector(str: &str) -> Vector2 {
    let (x_str, y_str) = str.split_once(", ").unwrap();
    Vector2(
        x_str.replace("X+", "").parse().unwrap(),
        y_str.replace("Y+", "").parse().unwrap(),
    )
}

fn read_input() -> Vec<ClawMachine> {
    let lines = std::io::stdin()
        .lines()
        .filter_map(|line| line.ok())
        .filter_map(|line| if line.is_empty() { None } else { Some(line) })
        .map(|line| line.trim().to_string())
        .collect::<Vec<String>>();
    let mut claw_machines = Vec::new();
    for machine_lines in lines.chunks(3) {
        let a_button = get_cost_vector(&machine_lines[0].replace("Button A: ", ""));
        let b_button = get_cost_vector(&machine_lines[1].replace("Button B: ", ""));

        let prize_line = machine_lines[2].replace("Prize: ", "");
        let (x_str, y_str) = prize_line.split_once(", ").unwrap();
        let prize = Vector2(
            x_str.replace("X=", "").parse().unwrap(),
            y_str.replace("Y=", "").parse().unwrap(),
        );

        claw_machines.push(ClawMachine {
            a: a_button,
            b: b_button,
            prize,
        });
    }

    claw_machines
}

fn find_path(machine: &ClawMachine) -> Option<(isize, isize)> {
    let start_position = Vector2::zero();

    let mut g_map = HashMap::new();
    let mut f_map = HashMap::new();

    g_map.insert(start_position, 0);
    f_map.insert(
        start_position,
        start_position.manhattan_distance(&machine.prize),
    );

    let mut visit: Vec<(Vector2, isize, isize)> = vec![(start_position, 0, 0)];

    while let Some((point, a_presses, b_presses)) = visit.pop() {
        if point.0 > machine.prize.0 || point.1 > machine.prize.1 {
            continue;
        }

        if point == machine.prize {
            return Some((a_presses, b_presses));
        }
        for (direction, cost, a, b) in [(machine.a, 1, 1, 0), (machine.b, 3, 0, 1)].iter() {
            let neighbor_point = point.add(direction);

            if !g_map.contains_key(&neighbor_point) {
                g_map.insert(neighbor_point, isize::MAX);
            }

            let new_distance = g_map.get(&point).unwrap() + cost;
            let neighbor_distance = g_map.get(&neighbor_point).unwrap();

            if new_distance < *neighbor_distance {
                *g_map.get_mut(&neighbor_point).unwrap() = new_distance;
                *f_map.entry(neighbor_point).or_insert(isize::MAX) =
                    new_distance + neighbor_point.manhattan_distance(&machine.prize);

                visit.push((neighbor_point, a_presses + a, b_presses + b));
            }
        }

        visit.sort_by(|a, b| {
            let a_f = f_map.get(&a.0).unwrap();
            let b_f = f_map.get(&b.0).unwrap();

            b_f.cmp(a_f)
        })
    }
    None
}

fn calc_path(machine: &ClawMachine, offset: isize) -> Option<(isize, isize)> {
    // Apply Cramer's rule
    let prize = machine.prize.add_scalar(offset);
    let det = machine.a.determinant(&machine.b);
    let a = prize.determinant(&machine.b) / det;
    let b = machine.a.determinant(&prize) / det;

    let pos = Vector2(
        machine.a.0 * a + machine.b.0 * b,
        machine.a.1 * a + machine.b.1 * b,
    );
    if pos == prize {
        Some((a, b))
    } else {
        None
    }
}

fn part1() -> Option<isize> {
    let machines = read_input();
    let result = machines
        .iter()
        .filter_map(|machine| find_path(machine))
        .map(|(a, b)| a * 3 + b)
        .sum();
    
    Some(result)
}

fn part2() -> Option<isize> {
    let machines = read_input();
    let result = machines
        .iter()
        .filter_map(|machine| calc_path(machine, 10000000000000))
        .map(|(a, b)| a * 3 + b)
        .sum();

    Some(result)
}

fn main() {
    println!("--- Day 13: Claw Contraption ---");
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
