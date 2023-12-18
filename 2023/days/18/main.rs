#[derive(Debug, Clone, Copy)]
enum Tile {
    Trench(u32),
    Terrain,
}

type Instruction = (char, i64, Tile);
type Position = (i64, i64);

fn read_input(flipped: bool) -> Vec<Instruction> {
    std::io::stdin()
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            let mut line_iter = line.trim().split(char::is_whitespace);
            let direction = line_iter.next().unwrap();
            let count = line_iter.next().unwrap().parse::<i64>().unwrap();
            let colour = line_iter.next().unwrap();
            let hex = &colour[2..colour.len() - 1];

            if flipped {
                let count = i64::from_str_radix(&hex[0..5], 16).unwrap();
                let direction = match &hex[5..] {
                    "0" => 'R',
                    "1" => 'D',
                    "2" => 'L',
                    "3" => 'U',
                    dir => panic!("Unknown direction {dir}"),
                };

                (direction, count, Tile::Trench(0))
            } else {
                (
                    direction.to_string().chars().next().unwrap(),
                    count,
                    Tile::Trench(u32::from_str_radix(hex, 16).unwrap()),
                )
            }
        })
        .collect()
}

fn create_map(instructions: &Vec<Instruction>) -> Vec<Position> {
    let mut current_position = (0_i64, 0_i64);
    let mut path = Vec::new();

    for &(direction, count, _) in instructions.iter() {
        match direction {
            'R' => {
                for _ in 0..count {
                    path.push(current_position.clone());
                    current_position = (current_position.0 + 1, current_position.1);
                }
            }
            'L' => {
                for _ in 0..count {
                    path.push(current_position.clone());
                    current_position = (current_position.0 - 1, current_position.1);
                }
            }
            'U' => {
                for _ in 0..count {
                    path.push(current_position.clone());
                    current_position = (current_position.0, current_position.1 - 1);
                }
            }
            'D' => {
                for _ in 0..count {
                    path.push(current_position.clone());
                    current_position = (current_position.0, current_position.1 + 1);
                }
            }
            _ => panic!("Unkown direction {direction}"),
        }
    }

    path
}

fn calculate_shape_area(path: &Vec<Position>) -> u64 {
  let mut area = 0.0;
  let n = path.len();

  for i in 0..n {
      let j = (i + 1) % n;
      let (xi, yi) = path[i];
      let (xj, yj) = path[j];

      // Add the area contribution of the edge itself
      area += xi as f64 * yj as f64 - xj as f64 * yi as f64;

      // Add the length of the edge
      area += (((xi - xj).pow(2) + (yi - yj).pow(2)) as f64).sqrt();
  }

  (area / 2_f64) as u64 + 1
}


fn part1() -> Option<u64> {
  let instructions = read_input(false);
  let path = create_map(&instructions);
  let result = calculate_shape_area(&path);
  
  Some(result)
}

fn part2() -> Option<u64> {
  let instructions = read_input(true);
  let path = create_map(&instructions);
  let result = calculate_shape_area(&path);
  
  Some(result)
}

fn main() {
    println!("--- Day 18: Lavaduct Lagoon ---");
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
