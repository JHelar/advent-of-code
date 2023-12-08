use std::{cmp::Ordering, collections::HashMap};

const VALUES: [char; 13] = [
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
];
const JOKER_VALUES: [char; 13] = [
    'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
];

#[derive(Debug, PartialEq, PartialOrd, Eq, Clone)]
enum HandType {
    FiveKind,
    FourKind,
    FullHouse,
    ThreeKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType {
    fn from_str(cards_str: String, joker: bool) -> Self {
        let mut cards_map: HashMap<char, u8> = HashMap::default();
        cards_str.chars().for_each(|c| {
            if let Some(card) = cards_map.get_mut(&c) {
                *card += 1;
            } else {
                cards_map.insert(c, 1);
            }
        });

        if joker {
            let mut additional_count = 0;
            if let Some(joker_count) = cards_map.get_mut(&'J') {
                // *best_count += *joker_count;
                additional_count = *joker_count;
                *joker_count = 0;
            }
            let (_, best_count) = cards_map.iter_mut().max_by(|a, b| a.1.cmp(&b.1)).unwrap();
            *best_count += additional_count;
        }

        let fives = cards_map.iter().find(|(_, count)| **count == 5).is_some();
        let fours = cards_map.iter().find(|(_, count)| **count == 4).is_some();
        let threes = cards_map.iter().find(|(_, count)| **count == 3).is_some();
        let twos = cards_map.iter().filter(|(_, count)| **count == 2).count();

        if fives {
            Self::FiveKind
        } else if fours {
            Self::FourKind
        } else if threes && twos == 1 {
            Self::FullHouse
        } else if threes {
            Self::ThreeKind
        } else if twos == 2 {
            Self::TwoPair
        } else if twos == 1 {
            Self::OnePair
        } else {
            Self::HighCard
        }
    }
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::FiveKind, Self::FiveKind) => Ordering::Equal,
            (Self::FiveKind, _) => Ordering::Greater,
            (_, Self::FiveKind) => Ordering::Less,

            (Self::FourKind, Self::FourKind) => Ordering::Equal,
            (Self::FourKind, _) => Ordering::Greater,
            (_, Self::FourKind) => Ordering::Less,

            (Self::FullHouse, Self::FullHouse) => Ordering::Equal,
            (Self::FullHouse, _) => Ordering::Greater,
            (_, Self::FullHouse) => Ordering::Less,

            (Self::ThreeKind, Self::ThreeKind) => Ordering::Equal,
            (Self::ThreeKind, _) => Ordering::Greater,
            (_, Self::ThreeKind) => Ordering::Less,

            (Self::TwoPair, Self::TwoPair) => Ordering::Equal,
            (Self::TwoPair, _) => Ordering::Greater,
            (_, Self::TwoPair) => Ordering::Less,

            (Self::OnePair, Self::OnePair) => Ordering::Equal,
            (Self::OnePair, _) => Ordering::Greater,
            (_, Self::OnePair) => Ordering::Less,

            _ => Ordering::Equal,
        }
    }
}

#[derive(Debug, Eq)]
struct Hand {
    cards: Vec<usize>,
    hand_type: HandType,
    bet: u32,
}

impl Hand {
    fn from_str(line: String, joker: bool) -> Self {
        let values = if joker { JOKER_VALUES } else { VALUES };
        let (cards_str, bet_str) = line.split_once(' ').unwrap();
        let bet = bet_str.parse().unwrap();

        let cards = cards_str
            .chars()
            .map(|card| values.iter().position(|c| *c == card).unwrap())
            .collect();

        let hand_type = HandType::from_str(cards_str.to_string(), joker);

        Self {
            cards,
            hand_type,
            bet,
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards && self.hand_type == other.hand_type && self.bet == other.bet
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let type_order = self.hand_type.cmp(&other.hand_type);

        match type_order {
            Ordering::Equal => {
                for i in 0..5 {
                    let a_card = self.cards[i];
                    let b_card = other.cards[i];

                    match a_card.cmp(&b_card) {
                        Ordering::Equal => continue,
                        Ordering::Greater => return Ordering::Less,
                        Ordering::Less => return Ordering::Greater,
                    };
                }
                Ordering::Equal
            }
            _ => type_order,
        }
    }
}

fn read_input() -> Vec<String> {
    std::io::stdin()
        .lines()
        .filter(|line| line.is_ok())
        .map(|line| line.unwrap())
        .collect()
}

fn part1() -> Option<u32> {
    let mut hands: Vec<_> = read_input()
        .iter()
        .map(|line| Hand::from_str(line.clone(), false))
        .collect();

    hands.sort();

    let result = hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| ((rank as u32) + 1_u32) * hand.bet)
        .sum();

    Some(result)
}

fn part2() -> Option<u32> {
    let mut hands: Vec<_> = read_input()
        .iter()
        .map(|line| Hand::from_str(line.clone(), true))
        .collect();

    hands.sort();

    let result = hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| ((rank as u32) + 1_u32) * hand.bet)
        .sum();

    Some(result)
}

fn main() {
    println!("--- Day 7: Camel Cards ---");
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
