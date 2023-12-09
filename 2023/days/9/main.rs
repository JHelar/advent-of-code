fn read_input() -> Vec<Vec<i32>> {
  std::io::stdin()
      .lines()
      .filter_map(|line| line.ok())
      .map(|line| line.trim().split_whitespace().map(|number| number.parse::<i32>().unwrap()).collect::<Vec<i32>>())
      .collect()
}

fn find_prev_next(sequence: &Vec<i32>) -> (i32, i32) {
  let new_sequence = sequence.windows(2).map(|nums| nums[1] - nums[0]).collect::<Vec<i32>>();
  
  if new_sequence.iter().any(|num| *num != 0) {
    let (next_prev, next_last) = find_prev_next(&new_sequence);
    (sequence.first().unwrap() - next_prev, next_last + sequence.last().unwrap())
  } else {
    (*sequence.first().unwrap(), *sequence.last().unwrap())
  }
}

fn part1() -> Option<i32> {
  let sequences = read_input();
  let result = sequences.iter().map(|sequence| find_prev_next(&sequence).1).sum();

  Some(result)
}

fn part2() -> Option<i32> {
  let sequences = read_input();
  let result = sequences.iter().map(|sequence| find_prev_next(&sequence).0).sum();

  Some(result)
}

fn main() {
  println!("--- Day 9: Mirage Maintenance ---");
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
