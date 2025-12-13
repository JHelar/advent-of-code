use std::{f32::consts::E, fmt::Display};

#[derive(Debug, Clone)]
struct Present {
    id: usize,
    shape: Vec<Vec<bool>>,
    width: usize,
    height: usize,
}

impl Display for Present {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Present({})", self.id)?;
        for row in self.shape.iter() {
            for tile in row.iter() {
                if *tile {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Present {
    fn new(id: &str) -> Option<Self> {
        let (id_str, _) = id.split_once(":").unwrap();
        if let Ok(id) = id_str.parse() {
            Some(Self {
                shape: Vec::default(),
                id,
                width: 0,
                height: 0,
            })
        } else {
            None
        }
    }

    fn add_shape_row(&mut self, row_str: &str) {
        let mut row = Vec::default();

        row_str.chars().for_each(|c| match c {
            '#' => row.push(true),
            '.' => row.push(false),
            _ => panic!("Unknown row shape {c}"),
        });

        self.width = row.len();
        self.height += 1;
        self.shape.push(row);
    }

    fn rot_90(&mut self) {
        let mut next_shape = self.shape.clone();

        for (row_index, row) in self.shape.iter().enumerate() {
            for (column_index, tile) in row.iter().enumerate() {
                next_shape[column_index][(row.len() - 1) - row_index] = *tile;
            }
        }
        self.shape = next_shape;
    }

    fn flip_horizontal(&mut self) {
        for row in self.shape.iter_mut() {
            row.reverse();
        }
    }

    fn flip_vertical(&mut self) {
        let mut next_shape = self.shape.clone();
        for (row_index, row) in self.shape.iter().enumerate() {
            next_shape[(self.shape.len() - 1) - row_index] = row.clone();
        }
        self.shape = next_shape;
    }
}

#[derive(Debug, Clone)]
struct Region {
    width: usize,
    height: usize,
    presents: Vec<usize>,
    shape: Vec<Vec<bool>>,
}

impl Display for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Region({},{})", self.width, self.height)?;
        for row in self.shape.iter() {
            for tile in row.iter() {
                if *tile {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Region {
    fn from_str(str: &str) -> Self {
        let (size_str, presents_str) = str.split_once(": ").unwrap();
        let (width_str, height_str) = size_str.split_once("x").unwrap();
        let width = width_str.parse().unwrap();
        let height = height_str.parse().unwrap();

        let mut shape = Vec::default();
        for _ in 0..height {
            let mut row = Vec::default();
            for _ in 0..width {
                row.push(false);
            }
            shape.push(row);
        }

        let presents = presents_str
            .split_whitespace()
            .map(|id| id.parse::<usize>().unwrap())
            .collect();

        Self {
            width,
            height,
            presents,
            shape,
        }
    }

    fn try_fit(&self, present: &Present) -> Vec<Vec<Vec<bool>>> {
        // Need to find all possible positions
        let mut fitting_shapes = Vec::new();
        for region_row_index in 0..=(self.height - present.height) {
            for region_column_index in 0..=(self.width - present.width) {
                let mut did_fit = true;
                let mut new_shape = self.shape.clone();

                'present: for present_row_index in 0..present.height {
                    let row_index = region_row_index + present_row_index;

                    for present_column_index in 0..present.width {
                        let column_index = region_column_index + present_column_index;
                        if new_shape[row_index][column_index]
                            && present.shape[present_row_index][present_column_index]
                        {
                            did_fit = false;
                            break 'present;
                        } else if present.shape[present_row_index][present_column_index] {
                            new_shape[row_index][column_index] = true;
                        }
                    }
                }
                if did_fit {
                    fitting_shapes.push(new_shape);
                }
            }
        }

        fitting_shapes
    }

    fn can_fit_all(&self, presents: &Vec<Present>) -> bool {
        let present_area = self
            .presents
            .iter()
            .enumerate()
            .map(|(present_index, count)| {
                count * (presents[present_index].width * presents[present_index].height)
            })
            .sum();
        let region_area = self.width * self.height;
        region_area >= present_area
    }

    fn can_optimally_fit_all(&self, presents: &Vec<Present>) -> bool {
        let present_tile_count = self
            .presents
            .iter()
            .enumerate()
            .map(|(present_index, count)| {
                let tile_count: usize = presents[present_index]
                    .shape
                    .iter()
                    .flat_map(|row| {
                        row.iter()
                            .filter_map(|tile| if *tile { Some(1usize) } else { None })
                    })
                    .sum();
                count * tile_count
            })
            .sum();
        let region_area = self.width * self.height;

        region_area >= present_tile_count
    }
}

fn try_pack(region: &Region, presents: &Vec<Present>) -> Result<Region, ()> {
    let mut my_region = region.clone();
    for (present_id, count) in my_region.presents.clone().into_iter().enumerate() {
        if count == 0 {
            continue;
        }
        // Rotate present 4 times
        let mut present = presents[present_id].clone();
        let mut shapes = Vec::new();
        for _ in 0..4 {
            for shape in my_region.try_fit(&present) {
                shapes.push(shape);
            }
            present.rot_90();
        }

        if shapes.len() == 0 {
            return Err(());
        }

        my_region.presents[present_id] -= 1;
        let mut success = true;
        for shape in shapes {
            my_region.shape = shape;
            if let Ok(finalized_region) = try_pack(&my_region, presents) {
                return Ok(finalized_region);
            } else {
                success = false
            }
        }
        if !success {
            return Err(());
        }
    }
    Ok(my_region)
}

fn try_fit_region(region: &Region, presents: &Vec<Present>) -> Result<Region, ()> {
    if region.can_fit_all(presents) {
        Ok(region.clone())
    } else if region.can_optimally_fit_all(presents) {
        Ok(region.clone())
    } else {
        //try_pack(region, presents)
        Err(())
    }
}

fn read_input() -> (Vec<Present>, Vec<Region>) {
    let mut presents = Vec::default();
    let mut regions = Vec::default();

    let mut current_present: Option<Present> = None;

    for line in std::io::stdin()
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| line.trim().to_string())
    {
        if line.is_empty() {
            if let Some(present) = current_present {
                presents.push(present);
                current_present = None;
            }
            continue;
        }
        if let Some(ref mut present) = current_present {
            present.add_shape_row(&line);
        } else if let Some(present) = Present::new(&line) {
            current_present = Some(present)
        } else {
            regions.push(Region::from_str(&line));
        }
    }

    (presents, regions)
}

fn part1() -> Option<isize> {
    let (presents, regions) = read_input();
    let mut sum = 0;
    for r in regions.iter() {
        if let Ok(new_region) = try_fit_region(r, &presents) {
            println!("{}", new_region);
            sum += 1;
        }
    }
    Some(sum)
}

fn part2() -> Option<isize> {
    None
}

fn main() {
    println!("--- Day 12: Christmas Tree Farm ---");
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
