#[derive(Debug)]
struct Equation {
  test_value: usize,
  values: Vec<usize>
}

enum Operator {
  Mul,
  Add,
  Concat
}

impl Operator {
    fn exec(&self, one: usize, another: usize) -> usize {
      match self {
          Operator::Add => one + another,
          Operator::Mul => one * another,
          Operator::Concat => format!("{one}{another}").parse().unwrap()
      }
    }
}

impl Equation {
    fn from_string(line: String) -> Self {
      let (test_value_str, values_str) = line.split_once(": ").unwrap();
      let values = values_str.split_whitespace().map(|value| value.parse().unwrap()).collect();
      let test_value = test_value_str.parse().unwrap();

      Self {
        test_value,
        values
      }
    }
}

fn validate_equation(target_value: usize, a: usize, values: &[usize], operators: &[Operator]) -> Result<(), ()> {
  if target_value == a && values.len() == 0 {
    return Ok(())
  } else if values.len() == 0 {
    return Err(())
  }

  let b = values[0];
  let new_values = &values[1..];

  for op in operators {
    let new_a = op.exec(a, b);
    match validate_equation(target_value, new_a, new_values, operators) {
      Ok(_) => return Ok(()),
      Err(_) => {}
    }
  }

  Err(())
}

fn read_input() -> Vec<Equation> {
  std::io::stdin()
      .lines()
      .filter_map(|line| line.ok())
      .map(Equation::from_string)
      .collect()
}

fn part1() -> Option<isize> {
  let equations = read_input();
  let mut result = 0;
  for eq in equations.iter() {
    match validate_equation(eq.test_value, eq.values[0], &eq.values[1..], &[Operator::Mul, Operator::Add]) {
        Ok(_) => {
          result += eq.test_value as isize;
        },
        Err(_) => {}
    }
  }
  Some(result)
}

fn part2() -> Option<isize> {
  let equations = read_input();
  let mut result = 0;
  for eq in equations.iter() {
    match validate_equation(eq.test_value, eq.values[0], &eq.values[1..], &[Operator::Mul, Operator::Add, Operator::Concat]) {
        Ok(_) => {
          result += eq.test_value as isize;
        },
        Err(_) => {}
    }
  }
  Some(result)
}

fn main() {
  println!("--- Day 7: Bridge Repair ---");
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
