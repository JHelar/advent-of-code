use std::collections::HashMap;

type Card = (Vec<u32>, Vec<u32>);

fn read_input() -> Vec<Card> {
    std::io::stdin()
        .lines()
        .filter(|line| line.is_ok())
        .map(|line| line.unwrap().trim().to_string())
        .map(|line| line.split_once(":").unwrap().1.trim().to_string())
        .map(|game_numbers| {
            let (my_numbers, winning_numbers) = game_numbers.split_once("|").unwrap();
            (
                my_numbers
                    .trim()
                    .split(" ")
                    .filter_map(|number_str| number_str.parse::<u32>().ok())
                    .collect::<Vec<u32>>(),
                winning_numbers
                    .trim()
                    .split(" ")
                    .filter_map(|number_str| number_str.parse::<u32>().ok())
                    .collect::<Vec<u32>>(),
            )
        })
        .collect()
}

fn part1() -> Option<u32> {
    let cards = read_input();
    let result: u32 = cards
        .iter()
        .map(|(mine, winning)| mine.iter().filter(|num| winning.contains(num)).count() as u32)
        .filter(|count| *count > 0)
        .map(|count| {
            if count == 1 {
                1
            } else {
                (0..(count - 1)).fold(1 as u32, |prev, _| prev * 2)
            }
        })
        .sum();

    Some(result)
}

fn part2() -> Option<u32> {
    let cards = read_input();
    let mut results: HashMap<usize, (u32, u32)> = cards
        .iter()
        .map(|(mine, winning)| mine.iter().filter(|num| winning.contains(num)).count() as u32)
        .enumerate()
        .map(|(id, count)| {
            (
                id,
                (
                    1,
                    count,
                ),
            )
        })
        .collect();

    for card_id in 0..cards.len() {
        let (card_count, card_result) = results.get(&card_id).unwrap().clone();

        (1..=card_result).for_each(|i| {
            if let Some(card) = results.get_mut(&(card_id + i as usize)) {
                card.0 += card_count;
            }
        })
    }

    let result = results.iter().map(|(_, (count, _))| count).sum();

    Some(result)
}

fn main() {
    println!("--- Day 4: Scratchcards ---");
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
