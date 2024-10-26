use std::io::Read;

fn read_input() -> Polymer {
    let mut string_buff = String::new();
    std::io::stdin().read_to_string(&mut string_buff).unwrap();
    Polymer::from_str(string_buff.trim())
}

struct Polymer {
    source: String,
    skip: Option<char>,
    reacted: String,
}

impl Polymer {
    fn from_str(str: &str) -> Self {
        Self {
            source: str.to_string(),
            reacted: str.to_string(),
            skip: None,
        }
    }
    fn get_units(&self) -> u32 {
        self.reacted.len() as u32
    }

    fn react(&mut self) -> u32 {
        self.reacted = self.source.clone();
        while let Some(_) = self.next() {};

        self.get_units()
    }

    fn react_with_skip(&mut self, unit: char) -> u32 {
        self.skip = Some(unit);

        self.react()
    }
}

impl Iterator for Polymer {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        let mut new_polymer = String::default();
        let polymer = self.reacted.clone();
        let mut chars = polymer.chars().peekable();

        let mut did_change = false;
        while let Some(char_left) = chars.next() {
            match self.skip {
                Some(char_skip) if char_skip.eq_ignore_ascii_case(&char_left) => {
                    did_change = true;
                    continue;
                }
                _ => match chars.peek() {
                    Some(&char_right) if (char_right as i8 - char_left as i8).abs() == 32 => {
                        chars.next()?;
                        did_change = true;
                    }
                    _ => {
                        new_polymer.push(char_left);
                    }
                },
            }
        }

        self.reacted = new_polymer;

        if did_change {
            Some(true)
        } else {
            None
        }
    }
}

fn part1() -> Option<u32> {
    let mut polymer = read_input();
    let result = polymer.react();
    Some(result)
}

fn part2() -> Option<u32> {
    let mut polymer = read_input();

    let result = (65..=90)
        .map(|unit| char::from_u32(unit).unwrap())
        .map(|skip_char| {
          polymer.react_with_skip(skip_char)
        })
        .min()
        .unwrap();

    Some(result)
}

fn main() {
    println!("--- Day 5: Alchemical Reduction ---");
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
