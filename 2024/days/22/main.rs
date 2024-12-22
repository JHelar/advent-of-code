use std::collections::HashMap;

fn read_input() -> Vec<u64> {
    std::io::stdin()
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| line.trim().parse().unwrap())
        .collect()
}

fn mix(secret: u64, number: u64) -> u64 {
    number ^ secret
}

fn prune(secret: u64) -> u64 {
    secret % 16777216
}

fn step(secret: u64) -> u64 {
    let step1 = prune(mix(secret, secret * 64));
    let step2 = prune(mix(step1, step1 / 32));
    let step3 = prune(mix(step2, step2 * 2048));

    step3
}

fn get_digit(secret: u64) -> u64 {
    secret % 10
}

fn part1() -> Option<u64> {
    let secrets = read_input();
    let result = secrets
        .into_iter()
        .map(|secret| (0..2000).fold(secret, |secret, _| step(secret)))
        .sum::<u64>();
    Some(result)
}

fn part2() -> Option<u64> {
    let secrets = read_input();
    let sequences = secrets
        .into_iter()
        .map(|secret| {
            let mut secret_acc = secret;
            let mut previous_digit = get_digit(secret_acc);
            let mut digits = Vec::new();
            for _ in 0..=2000 {
                secret_acc = step(secret_acc);
                let digit = get_digit(secret_acc);
                let diff = (digit as i64) - (previous_digit as i64);
                previous_digit = digit;
                digits.push((digit, diff));
            }
            digits
        })
        .collect::<Vec<Vec<(u64, i64)>>>();

    let mut all_sequences: HashMap<Vec<i64>, u64> = HashMap::new();
    for sequence in sequences.iter() {
        let mut this_sequences: HashMap<Vec<i64>, u64> = HashMap::new();
        for subsequence in sequence.windows(4) {
          let sequence_key = subsequence.iter().map(|(_, diff)| *diff).collect();

          if this_sequences.contains_key(&sequence_key) {
            continue;
          }
          this_sequences.insert(sequence_key, subsequence[3].0);
        }
        for (k, v) in this_sequences.into_iter() {
            *all_sequences.entry(k).or_insert(0) += v;
        }
    }
    let best = all_sequences
        .iter()
        .max_by(|a, b| a.1.cmp(b.1))
        .expect("Should have a best sequence");

    Some(*best.1)
}

fn main() {
    println!("--- Day 22: Monkey Market ---");
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
