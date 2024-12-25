use std::collections::{btree_map::Keys, HashSet};

fn get_lock_code(lock: &Vec<String>) -> Vec<usize> {
  let mut code = vec![0; 5];
  for column in 0..5 {
    for row in lock[1..].iter() {
        if &row[column..column + 1] == "#" {
          code[column] += 1;
        }
    }
  }
  code
}

fn get_key_code(key: &Vec<String>) -> Vec<usize> {
  let mut code = vec![0; 5];
  for column in 0..5 {
    for row in key[0..6].iter() {
        if &row[column..column + 1] == "#" {
          code[column] += 1;
        }
    }
  }
  code
}

fn key_fits(key: &Vec<usize>, lock: &Vec<usize>) -> bool {
  for column in 0..5 {
      let key_col = key[column];
      let lock_col = lock[column];

      let val = key_col as isize + lock_col as isize;
      if val > 5 {
        return false;
      }
  }
  return true
}

fn read_input() -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {

  let mut keys = Vec::new();
  let mut locks = Vec::new();

  let mut current = Vec::new();

  for row in std::io::stdin()
  .lines()
  .filter_map(|line| line.ok())
  .map(|line| line.trim().to_string()) {
    if row.is_empty() {
      if current[0] == "#####" {
        locks.push(get_lock_code(&current));
      } else {
        keys.push(get_key_code(&current));
      }
      current.clear();
      continue;
    }
    current.push(row);
  }
  if current[0] == "#####" {
    locks.push(get_lock_code(&current));
  } else {
    keys.push(get_key_code(&current));
  }
  (locks, keys)
}

fn part1() -> Option<isize> {
  let (locks, keys) = read_input();
  let mut result = 0;
  for lock in locks.iter() {
      for key in keys.iter() {
        if key_fits(key, lock) {
          result += 1;
        }
      }
  }
  Some(result)
}

fn part2() -> Option<isize> {
  None
}

fn main() {
  println!("--- Day 25: Code Chronicle ---");
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
