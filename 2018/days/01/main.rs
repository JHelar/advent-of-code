use std::{collections::HashSet, iter::Sum};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Frequency {
    Positive(u32),
    Negative(u32),
}

impl Frequency {
    fn from_string(str: &str) -> Self {
        let sign = &str[..1];
        let num = &str[1..].parse::<u32>().unwrap();

        match sign {
            "+" => Self::Positive(*num),
            "-" => Self::Negative(*num),
            _ => panic!("Invalid sign {sign}"),
        }
    }

    fn change_with(&self, with: Frequency) -> Self {
        match self {
            Frequency::Positive(self_value) => match with {
                Frequency::Positive(with_value) => Frequency::Positive(self_value + with_value),
                Frequency::Negative(with_value) => {
                    if *self_value < with_value {
                        Frequency::Negative(with_value - self_value)
                    } else {
                        Frequency::Positive(self_value - with_value)
                    }
                }
            },
            Frequency::Negative(self_value) => match with {
                Frequency::Negative(with_value) => Frequency::Negative(self_value + with_value),
                Frequency::Positive(with_value) => {
                    if *self_value < with_value {
                        Frequency::Positive(with_value - self_value)
                    } else {
                        Frequency::Negative(self_value - with_value)
                    }
                }
            },
        }
    }

    fn get_value(&self) -> Option<u32> {
        match self {
            Frequency::Positive(value) => Some(*value),
            _ => None,
        }
    }
}

impl Sum for Frequency {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|acc, frequency| acc.change_with(frequency))
            .unwrap()
    }
}

fn read_input() -> Vec<Frequency> {
    std::io::stdin()
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| Frequency::from_string(line.trim().to_string().as_str()))
        .collect()
}

fn part1() -> Option<u32> {
    let frequencies = read_input();
    let result = frequencies.into_iter().sum::<Frequency>();
    result.get_value()
}

fn part2() -> Option<u32> {
    let frequencies = read_input();
    let mut mem: HashSet<Frequency> = HashSet::default();
    let mut acc = Frequency::Positive(0);
    for frequency in frequencies.into_iter().cycle() {
        acc = acc.change_with(frequency);
        if mem.contains(&acc) {
            break;
        }
        mem.insert(acc.clone());
    }

    acc.get_value()
}

fn main() {
    println!("--- Day 1: Chronal Calibration ---");
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
