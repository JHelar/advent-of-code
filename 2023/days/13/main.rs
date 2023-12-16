type Map = Vec<Vec<Tile>>;

#[derive(Debug, PartialEq, Clone, Copy, Eq)]
enum Tile {
    Ash,
    Rock,
}

impl Tile {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Ash,
            '#' => Self::Rock,
            _ => panic!("Unknown tile type: {c}"),
        }
    }
}

fn are_reflections(window: Vec<Vec<Tile>>, smudge: bool) -> (bool, bool) {
    let one = window[0].iter();
    let another = window[1].iter();

    let mut errors = 0;
    for (a, b) in std::iter::zip(one, another) {
        if *a != *b {
            if smudge && errors == 0 {
                errors += 1;
            } else {
                return (false, errors > 1);
            }
        }
    }
    (true, errors == 1)
}

fn are_mirrors(left_start: usize, right_start: usize, map: &Map, smudge: bool) -> bool {
    if (right_start - left_start) % 2 == 0 {
        return false;
    }

    let mut left = left_start;
    let mut right = right_start;
    let mut errors = 0;

    while left < right {
        let window = vec![map[left].clone(), map[right].clone()];
        let (is_reflection, with_error) = are_reflections(window, smudge);
        if !is_reflection {
            return false;
        }
        if smudge && with_error {
            if errors == 0 {
                errors += 1;
            } else {
                return false;
            }
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
                let tiles = line.trim().chars().map(Tile::from_char).collect();
                current_map.push(tiles);
            }
        });
    maps.push(current_map.clone());
    maps
}

fn get_mirror_row(
    map: &Map,
    (ignore_top, ignore_bottom): (usize, usize),
    smudge: bool,
) -> Option<(usize, usize)> {
    for bottom_y in (1..map.len()).rev() {
        let top_y = 0;
        if ignore_top == top_y && ignore_bottom == bottom_y {
            continue;
        }

        let window = vec![map[top_y].clone(), map[bottom_y].clone()];
        if are_reflections(window, smudge).0 && are_mirrors(top_y, bottom_y, map, smudge) {
            return Some((top_y, bottom_y));
        }
    }
    for top_y in 0..map.len() - 1 {
        let bottom_y = map.len() - 1;
        if ignore_top == top_y && ignore_bottom == bottom_y {
            continue;
        }

        let window = vec![map[top_y].clone(), map[bottom_y].clone()];
        if are_reflections(window, smudge).0 && are_mirrors(top_y, bottom_y, map, smudge) {
            return Some((top_y, bottom_y));
        }
    }
    None
}

fn get_mirror_column(
    map: &Map,
    (ignore_left, ignore_right): (usize, usize),
    smudge: bool,
) -> Option<(usize, usize)> {
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

        if ignore_left == left_x && ignore_right == right_x {
            continue;
        }

        let window = vec![flipped_map[left_x].clone(), flipped_map[right_x].clone()];
        if are_reflections(window.to_vec(), smudge).0
            && are_mirrors(left_x, right_x, &flipped_map, smudge)
        {
            return Some((left_x, right_x));
        }
    }
    for left_x in 0..flipped_map.len() - 1 {
        let right_x = flipped_map.len() - 1;
        let window = vec![flipped_map[left_x].clone(), flipped_map[right_x].clone()];

        if ignore_left == left_x && ignore_right == right_x {
            continue;
        }

        if are_reflections(window.to_vec(), smudge).0
            && are_mirrors(left_x, right_x, &flipped_map, smudge)
        {
            return Some((left_x, right_x));
        }
    }
    None
}

fn part1() -> Option<u32> {
    let maps = read_input();
    let mut sum = 0;
    for map in maps.iter() {
        sum += if let Some((top_y, bottom_y)) = get_mirror_row(map, (usize::MAX, usize::MAX), false)
        {
            let diff = (bottom_y - top_y) / 2;
            let rows_to_top = diff + top_y + 1;

            100 * rows_to_top as u32
        } else if let Some((left_x, right_x)) =
            get_mirror_column(map, (usize::MAX, usize::MAX), false)
        {
            let diff = (right_x - left_x) / 2;
            let columns_to_left = diff + left_x + 1;

            columns_to_left as u32
        } else {
            0
        };
    }

    Some(sum)
}

fn part2() -> Option<u32> {
    let maps = read_input();
    let mut sum = 0;

    for map in maps.iter() {
        let previous_rows = get_mirror_row(map, (usize::MAX, usize::MAX), false)
            .unwrap_or((usize::MAX, usize::MAX));
        let current_rows =
            get_mirror_row(map, previous_rows, true).unwrap_or((usize::MAX, usize::MAX));

        let previous_columns = get_mirror_column(map, (usize::MAX, usize::MAX), false)
            .unwrap_or((usize::MAX, usize::MAX));
        let current_columns =
            get_mirror_column(map, previous_columns, true).unwrap_or((usize::MAX, usize::MAX));

        sum += if previous_rows != current_rows && current_rows != (usize::MAX, usize::MAX) {
            let diff = (current_rows.1 - current_rows.0) / 2;
            let rows_to_top = diff + current_rows.0 + 1;

            100 * rows_to_top as u32
        } else if previous_columns != current_columns && current_columns != (usize::MAX, usize::MAX)
        {
            let diff = (current_columns.1 - current_columns.0) / 2;
            let columns_to_left = diff + current_columns.0 + 1;

            columns_to_left as u32
        } else {
            0
        };
    }

    Some(sum)
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
