fn read_input() -> Vec<String> {
    std::io::stdin()
        .lines()
        .filter(|line| line.is_ok())
        .map(|line| line.unwrap().trim().to_string())
        .collect()
}

const NUM_STR: [&str; 18] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
    "5", "6", "7", "8", "9",
];

fn part1() -> Option<u32> {
    let input = read_input();
    let sum = input.iter().fold(0_u32, |acc, line| {
        let chars_iter = line.chars().collect::<Vec<char>>();
        let head = chars_iter.iter().find(|c| c.is_numeric()).unwrap();
        let tail = chars_iter.iter().rev().find(|c| c.is_numeric()).unwrap();

        acc + format!("{head}{tail}").parse::<u32>().unwrap()
    });

    Some(sum)
}

fn part2() -> Option<u32> {
    let input = read_input();
    let sum = input
        .iter()
        .map(|line| {
            let nums = NUM_STR
                .iter()
                .enumerate()
                .flat_map(|(num_i, num_str)| {
                    line.match_indices(num_str)
                        .map(move |(i, _)| (i as i32, (num_i % 9) + 1))
                })
                .collect::<Vec<(i32, usize)>>();

            let (head, _) = nums.iter().fold((0, i32::MAX), |acc, i| {
                if i.0 < acc.1 {
                    return (i.1, i.0);
                }
                acc
            });

            let (tail, _) = nums.iter().fold((0, -1), |acc, i| {
                if i.0 > acc.1 {
                    return (i.1, i.0);
                }
                acc
            });

            format!("{head}{tail}").parse::<u32>().unwrap()
        })
        .sum();

    Some(sum)
}

fn main() {
    println!("--- Day 1: Trebuchet?! ---");
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
