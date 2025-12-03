#[derive(Debug)]
struct BatteryBank {
    batteries: Vec<u64>,
}

impl BatteryBank {
    fn from_str(str: &str) -> Option<Self> {
        if str.is_empty() {
            None
        } else {
            Some(Self {
                batteries: str.chars().filter_map(|c| c.to_digit(10)).map(|d| d as u64).collect(),
            })
        }
    }

    fn largest_pair(&self, jolt_size: usize) -> Option<u64> {
        let mut with_index = Vec::default();
        for (index, battery) in self.batteries.iter().enumerate() {
            with_index.push((index, *battery));
        }

        with_index.sort_by(|a, b| b.1.cmp(&a.1));

        let mut largest_pair: Option<(usize, u64)> = None;
        let mut largest_battery = 0;
        for &pair in with_index.iter() {
            if (with_index.len() - pair.0) >= jolt_size && pair.1 > largest_battery {
              largest_pair = Some(pair);
              largest_battery = pair.1;
            }
        }

        
        let mut look_at_pair = largest_pair.expect("Should have found a fitting pair");
        let mut jolt_sum = look_at_pair.1 * 10_u64.pow((jolt_size - 1) as u32);

        for size in (1..jolt_size).rev() {
          for &(index, battery) in with_index.iter() {
              if look_at_pair.0 < index && (with_index.len() - index) >= size {
                look_at_pair = (index, battery);
                jolt_sum += battery * 10_u64.pow((size - 1) as u32);
                break;
              }
          }   
        }

        Some(jolt_sum)
    }
}

fn read_input() -> Vec<BatteryBank> {
    std::io::stdin()
        .lines()
        .filter_map(|line| line.ok())
        .filter_map(|line| BatteryBank::from_str(&line))
        .collect()
}

fn part1() -> Option<u64> {
    let banks = read_input();
    let mut sum = 0;
    for bank in banks {
        let jolt = bank.largest_pair(2).expect("Should have a largest pair");
        sum += jolt
    }
    Some(sum)
}

fn part2() -> Option<u64> {
    let banks = read_input();
    let mut sum = 0;
    for bank in banks {
        let jolt = bank.largest_pair(12).expect("Should have a largest pair");
        sum += jolt
    }
    Some(sum)
}

fn main() {
    println!("--- Day 3: Lobby ---");
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
