fn read_input() -> (Vec<u64>, Vec<u64>) {
    let mut lines_iter = std::io::stdin()
        .lines()
        .filter(|line| line.is_ok())
        .map(|line| line.unwrap().trim().to_string());

    let times = lines_iter.next().unwrap()["Time:".len()..]
        .trim()
        .split(char::is_whitespace)
        .filter_map(|number| number.parse::<u64>().ok())
        .collect::<Vec<u64>>();
    let distances = lines_iter.next().unwrap()["Distance:".len()..]
        .trim()
        .split(char::is_whitespace)
        .filter_map(|number| number.parse::<u64>().ok())
        .collect::<Vec<u64>>();

    (times, distances)
}

fn get_duration(distance: u64, race_time: u64) -> f64 {
    ((race_time as f64 - ((race_time.pow(2) - 4 * distance) as f64).sqrt()) / 2 as f64).floor()
}

fn get_max_duration(race_time: u64) -> f64 {
    race_time as f64 / 2_f64
}

fn part1() -> Option<u64> {
    let (times, distances) = read_input();
    
    let races = times
        .iter()
        .zip(distances.iter())
        .map(|(a, b)| (*a, *b))
        .collect::<Vec<(u64, u64)>>();

    let result: f64 = races
        .iter()
        .map(|(race_time, record_distance)| {
            let record_duration = get_duration(*record_distance, *race_time);
            let max_duration = get_max_duration(*race_time);
            ((max_duration - record_duration) * 2_f64) - 1_f64
        })
        .product();

    Some(result as u64)
}

fn part2() -> Option<u64> {
    let (times, distances) = read_input();
    let race_time = times.iter().fold("".to_string(),|prev, time| format!("{prev}{time}")).parse::<u64>().unwrap();
    let record_distance = distances.iter().fold("".to_string(),|prev, distance| format!("{prev}{distance}")).parse::<u64>().unwrap();

    let record_duration = get_duration(record_distance, race_time);
    let max_duration = get_max_duration(race_time);
    let result = (((max_duration - record_duration) * 2_f64) - 1_f64) as u64;

    Some(result)
}

fn main() {
    println!("--- Day 6: Wait For It ---");
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
