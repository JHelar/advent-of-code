type Map = Vec<Vec<Tile>>;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Tile {
    Ash,
    Rock,
}

impl Tile {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Rock,
            '#' => Self::Ash,
            _ => panic!("Unknown tile type: {c}"),
        }
    }
}

fn are_reflections(window: Vec<Vec<Tile>>) -> bool {
    let one = window[0].iter();
    let another = window[1].iter();

    for (a, b) in std::iter::zip(one, another) {
        if *a != *b {
            return false;
        }
    }
    true
}

fn are_mirrors(left_start: usize, right_start: usize, map: &Map) -> bool {
    if (right_start - left_start) % 2 == 0 {
      return false;
    }

    let mut left = left_start;
    let mut right = right_start;
    while left < right {
        let window = vec![map[left].clone(), map[right].clone()];
        if !are_reflections(window) {
            return false;
        }

        left += 1;
        right -= 1;
    }

    return true;
}

fn read_input() -> Vec<Map> {
    let mut maps = Vec::new();
    let mut current_map: Map = Vec::new();

    std::io::stdin()
        .lines()
        .filter_map(|line| line.ok())
        .for_each(|line| {
            if line.is_empty() {
                if !current_map.is_empty() {
                    maps.push(current_map.clone());
                    current_map.clear();
                }
            } else {
                current_map.push(line.trim().chars().map(Tile::from_char).collect());
            }
        });
    maps.push(current_map.clone());
    maps
}

fn get_mirror_row(map: &Map) -> Option<(usize, usize)> {
    for bottom_y in (1..map.len()).rev() {
        let top_y = 0;
        let window = vec![map[top_y].clone(), map[bottom_y].clone()];
        if are_reflections(window) && are_mirrors(top_y, bottom_y, map) {
            return Some((top_y, bottom_y));
        }
    }
    for top_y in 0..map.len() - 1 {
        let bottom_y = map.len() - 1;
        let window = vec![map[top_y].clone(), map[bottom_y].clone()];
        if are_reflections(window) && are_mirrors(top_y, bottom_y, map) {
            return Some((top_y, bottom_y));
        }
    }
    None
}

fn get_mirror_column(map: &Map) -> Option<(usize, usize)> {
    let mut flipped_map = Vec::new();
    for x in 0..map[0].len() {
        let mut column = Vec::new();
        for row in map.iter() {
            column.push(row[x].clone());
        }
        flipped_map.push(column);
    }

    for right_x in (1..flipped_map.len()).rev() {
        let left_x = 0;
        let window = vec![flipped_map[left_x].clone(), flipped_map[right_x].clone()];
        if are_reflections(window.to_vec()) && are_mirrors(left_x, right_x, &flipped_map) {
            return Some((left_x, right_x));
        }
    }
    for left_x in 0..flipped_map.len() - 1 {
        let right_x = flipped_map.len() - 1;
        let window = vec![flipped_map[left_x].clone(), flipped_map[right_x].clone()];

        if are_reflections(window.to_vec()) && are_mirrors(left_x, right_x, &flipped_map) {
            return Some((left_x, right_x));
        }
    }
    None
}

fn print_map(map: &Map) {
  for row in map.iter() {
      for tile in row {
          match tile {
              Tile::Ash => print!("."),
              Tile::Rock => print!("#")
          }
      }
      print!("\n");
  }
}

fn part1() -> Option<u32> {
    let maps = read_input();
    let mut sum = 0;
    for (i, map) in maps.iter().enumerate() {
        let mut found_row = false;
        let mut found_col = false;

        if let Some((top_y, bottom_y)) = get_mirror_row(map) {
            found_row = true;
            let diff = (bottom_y - top_y) / 2;
            let rows_to_top = diff + top_y + 1;

            sum += 100 * rows_to_top as u32;
        }
        if let Some((left_x, right_x)) = get_mirror_column(map) {
            found_col = true;
            let diff = (right_x - left_x) / 2;
            let columns_to_left = diff + left_x + 1;

            sum += columns_to_left as u32;
        }

        if !found_row && !found_col {
            print_map(map);
            panic!("{i}");
        }
        if found_row && found_col {
          print_map(map);
          panic!("{i}");
        }
    }

    Some(sum)
}

fn part2() -> Option<u32> {
    None
}

fn main() {
    println!("--- Day 13: Point of Incidence ---");
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
