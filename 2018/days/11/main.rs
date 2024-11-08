use rayon::iter::{IntoParallelIterator, ParallelIterator};

type Position = (u32, u32);
type Grid = Vec<i32>;

const GRID_SIZE: u32 = 300;

fn get_power_level((x, y): Position, serial_number: u32) -> i32 {
    let rack_id = x + 10;
    let power_level = (rack_id * y + serial_number) * rack_id;
    let digit = (power_level as i32 / 100) % 10;
    digit - 5
}

fn read_input() -> u32 {
    let mut line = String::new();

    let _ = std::io::stdin().read_line(&mut line);

    line.trim().parse().unwrap()
}

fn create_grid(serial_number: u32) -> Grid {
    let mut grid = Vec::default();
    for y in 1..=GRID_SIZE {
        for x in 1..=GRID_SIZE {
            let position = (x, y);
            grid.push(get_power_level(position, serial_number));
        }
    }

    grid
}

fn get_window_power_level_at(grid: &Grid, (x, y): Position, window_size: u32) -> i32 {
    let mut sum = 0;
    for y_window in 0..window_size {
        let start_index = (x - 1 + ((y + y_window) - 1) * GRID_SIZE) as usize;
        let end_index = ((x - 1) + window_size + ((y + y_window) - 1) * GRID_SIZE) as usize;
        sum += grid[start_index..end_index].iter().sum::<i32>();
    }
    sum
}

fn get_best_power_level_for_window(grid: &Grid, window_size: u32) -> (Position, i32) {
    let mut best_power_level = i32::min_value();
    let mut best_position = (0, 0);

    for y in 1..=(GRID_SIZE - window_size) {
        for x in 1..=(GRID_SIZE - window_size) {
            let position = (x, y);
            let power_level = get_window_power_level_at(&grid, position, window_size);
            if power_level > best_power_level {
                best_power_level = power_level;
                best_position = position;
            }
        }
    }
    (best_position, best_power_level)
}

fn part1() -> Option<String> {
    let serial_number = read_input();
    let grid = create_grid(serial_number);

    let window_size = 3;
    let (best_position, _) = get_best_power_level_for_window(&grid, window_size);
    Some(format!("{},{}", best_position.0, best_position.1))
}

fn part2() -> Option<String> {
    let serial_number = read_input();
    let grid = create_grid(serial_number);

    let (_, best_position, best_window_size) = (1..=GRID_SIZE)
        .into_par_iter()
        .map(|window_size| {
            let (position, power_level) = get_best_power_level_for_window(&grid, window_size);
            (power_level, position, window_size)
        })
        .max_by(|a, b| a.0.cmp(&b.0))
        .unwrap();

    Some(format!(
        "{},{},{}",
        best_position.0, best_position.1, best_window_size
    ))
}

fn main() {
    println!("--- Day 11: Chronal Charge ---");
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
