use hashbrown::HashSet;

const MAX_TOWEL_LEN: usize = 64;

type PatternSet = HashSet<String>;

fn read_input() -> (Counter, Vec<String>) {
    let mut patterns: PatternSet = HashSet::new();
    let mut towels = Vec::new();

    let mut read_storage = true;
    for line in std::io::stdin()
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| line.trim().to_string())
    {
        if line.is_empty() {
            read_storage = false;
            continue;
        }

        if read_storage {
            patterns = line
                .split(", ")
                .map(|towel| towel.to_string())
                .collect();
        } else {
            towels.push(line);
        }
    }

    (Counter::new(patterns), towels)
}

struct Counter {
    patterns: PatternSet,
    max_pat_len: usize,
    tally: [usize; MAX_TOWEL_LEN],
}

impl Counter {
    fn new(patterns: PatternSet) -> Self {
        let max_pat_len = patterns.iter().map(|p| p.len()).max().expect("no patterns");
        Self { patterns, max_pat_len, tally: [0; MAX_TOWEL_LEN] }
    }

    fn count_permutations(&mut self, towel: &String) -> usize {
        let towel_len = towel.len();
        self.tally.fill(0);
        self.tally[0] = 1;

        for start in 0..towel_len {
            if self.tally[start] > 0 {
                for pat_len in 1..=self.max_pat_len.min(towel_len - start) {
                    let end = start + pat_len;
                    if self.patterns.contains(&towel[start..end]) {
                        self.tally[end] += self.tally[start];
                    }
                }
            }
        }

        self.tally[towel_len]
    }
}

fn part1() -> Option<usize> {
    let (mut counter, towels) = read_input();
    let completed = towels.iter().filter(|towel| counter.count_permutations(towel) > 0).count();
    Some(completed)
}

fn part2() -> Option<usize> {
  let (mut counter, towels) = read_input();
  let sum = towels.iter().map(|towel| counter.count_permutations(towel)).sum();
  Some(sum)
}

fn main() {
    println!("--- Day 19: Linen Layout ---");
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
