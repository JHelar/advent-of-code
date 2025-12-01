#[derive(Debug)]
enum DialRotation {
  Left(i64),
  Right(i64)
}

impl DialRotation {
  fn from_str(str: &str) -> Self {
    let (direction, amount_str) = str.split_at(1);
    let amount = amount_str.parse::<i64>().unwrap();

    match direction {
        "L" => Self::Left(amount),
        "R" => Self::Right(amount),
        _ => panic!("Unexpected direction '{direction}'")
    }
  }

  fn count(&self) -> i64 {
    match self {
        Self::Left(count) => *count,
        Self::Right(count) => *count,
    }
  }

  fn direction(&self) -> i64 {
    match self {
        Self::Left(count) => -1,
        Self::Right(count) => 1,
    }
  }
}

struct Dial {
  rotation_state: i64,
  dial_size: i64,
  hits: i64,
  count_rotation_hits: bool
}

impl Dial {
    fn new(initial_rotation: i64, dial_size: i64, count_rotation_hits: bool) -> Self   {
      Self { rotation_state: initial_rotation, dial_size, hits: 0, count_rotation_hits }
    }

    fn rotate(&mut self, rotation: &DialRotation) {
      let count = rotation.count();
      let direction = rotation.direction();
      let mut rotations = 0;
      let mut started_at_zero = self.rotation_state == 0;
      
      for index in 0..count {
        let new_state = self.rotation_state + (1 * direction);
        if new_state > self.dial_size {
          self.rotation_state = 0;
          if started_at_zero || index == count - 1 {
            started_at_zero = false;
          }else {
            rotations += 1;
          }
        } else if new_state < 0 {
          self.rotation_state = self.dial_size;
          if started_at_zero {
            started_at_zero = false;
          }else {
            rotations += 1;
          }
        } else {
          self.rotation_state = new_state;
          started_at_zero = false;
        }
      }

      if self.count_rotation_hits {
        self.hits += rotations;
      }
      if self.rotation_state == 0 {
        self.hits += 1;
      }
    }
}

fn read_input() -> Vec<DialRotation> {
  std::io::stdin()
      .lines()
      .filter_map(|line| line.ok())
      .map(|line| DialRotation::from_str(&line))
      .collect()
}

fn part1() -> Option<i64> {
  let rotations = read_input();
  let mut dial = Dial::new(50, 99, false);
  for rotation in rotations.iter() {
      dial.rotate(rotation);
  }

  Some(dial.hits)
}

fn part2() -> Option<i64> {
  let rotations = read_input();
  let mut dial = Dial::new(50, 99, true);
  for rotation in rotations.iter() {
      dial.rotate(rotation);
  }

  Some(dial.hits)
}

fn main() {
  println!("--- Day 1: Secret Entrance ---");
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
