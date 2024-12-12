use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use vector2::{Vector2, DOWN, LEFT, RIGHT, UP};

const DIRECTIONS: [Vector2; 4] = [UP, DOWN, LEFT, RIGHT];
const CORNERS: [[Vector2; 2]; 4] = [[UP, RIGHT], [UP, LEFT], [DOWN, RIGHT], [DOWN, LEFT]];

mod vector2;

type PlotMap = Vec<Vec<usize>>;

#[derive(Debug)]
struct Region {
    plot_name: char,
    region_index: usize,
    points: Vec<Vector2>,
    perimeter: Vec<Vector2>,
    corners: HashMap<Vector2, usize>,
}

impl Region {
    fn set_perimeter(&mut self, map: &PlotMap) {
        self.perimeter.clear();

        for point in self.points.iter() {
            for perimeter_point in DIRECTIONS.iter().filter_map(|direction| {
                let neighbor_point = point.add(direction);
                if let Some(row) = map.get(neighbor_point.1 as usize) {
                    if let Some(region_index) = row.get(neighbor_point.0 as usize) {
                        if *region_index != self.region_index {
                            Some(neighbor_point)
                        } else {
                            None
                        }
                    } else {
                        Some(neighbor_point)
                    }
                } else {
                    Some(neighbor_point)
                }
            }) {
                self.perimeter.push(perimeter_point);
            }
        }
    }

    fn set_sides(&mut self, map: &PlotMap) {
        self.set_perimeter(map);

        self.corners.clear();

        let mut perimeter_with_corners = HashSet::new();
        for perimeter_point in self.perimeter.iter() {
            perimeter_with_corners.insert(perimeter_point.clone());

            for direction in DIRECTIONS.iter() {
                let corner_point = perimeter_point.add(direction);
                if self.points.contains(&corner_point) {
                    continue;
                }

                let count = CORNERS.iter().filter(|[one, two]| {
                    let body_check_1 = corner_point.add(&one.add(two));
                    let body_check_2 = body_check_1.mul_scalar(-1);

                    let perimeter_point_1 = corner_point.add(one);
                    let perimeter_point_2 = corner_point.add(two);

                    let is_two_perimeters = self.perimeter.contains(&perimeter_point_1)
                    && self.perimeter.contains(&perimeter_point_2);

                    if !is_two_perimeters {
                        return false;
                    }

                    let is_body_1 = self.points.contains(&body_check_1)
                        && perimeter_point_1.add(two) == body_check_1
                        && perimeter_point_2.add(one) == body_check_1;

                    let is_body_2 = self.points.contains(&body_check_2)
                        && perimeter_point_1.add(two) == body_check_2
                        && perimeter_point_2.add(one) == body_check_2;

                    return is_body_1 || is_body_2;
                }).count();
                if count > 0 {
                    perimeter_with_corners.insert(corner_point);
                    self.corners.insert(corner_point, count);
                }
            }
        }

        for perimeter_point in perimeter_with_corners.into_iter() {
            let internal_corners = CORNERS
                .iter()
                .filter(|[a, b]| {
                    let a_point = perimeter_point.add(a);
                    let b_point = perimeter_point.add(b);

                    if self.points.contains(&a_point) && self.points.contains(&b_point) {
                        true
                    } else {
                        false
                    }
                })
                .count();

            if internal_corners > 0 {
                *self.corners.entry(perimeter_point).or_insert(0) += internal_corners;
            }
        }
    }

    fn get_perimiter_price(&self) -> isize {
        (self.points.len() * self.perimeter.len()) as isize
    }

    fn get_side_price(&self) -> isize {
        (self.points.len() * self.corners.values().map(|sides| *sides).sum::<usize>()) as isize
    }
}

impl Display for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let min_x = self.perimeter.iter().map(|p| p.0).min().unwrap();
        let max_x = self.perimeter.iter().map(|p| p.0).max().unwrap();
        let min_y = self.perimeter.iter().map(|p| p.1).min().unwrap();
        let max_y = self.perimeter.iter().map(|p| p.1).max().unwrap();
        write!(f, "Region: {}, Len: {}, Corners: {}\n", self.plot_name, self.points.len(), self.corners.values().map(|sides| *sides).sum::<usize>())?;
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if self.corners.contains_key(&Vector2(x, y)) {
                    write!(f, "+")?
                } else if self.perimeter.contains(&Vector2(x, y)) {
                    write!(f, ".")?
                } else if self.points.contains(&Vector2(x, y)) {
                    write!(f, "{}", self.plot_name)?
                } else {
                    write!(f, " ")?
                }
            }
            write!(f, "\n")?
        }
        Ok(())
    }
}

fn read_input() -> (PlotMap, Vec<Region>) {
    let mut all_plot_points = HashMap::new();
    let mut plot_map: PlotMap = Vec::new();

    for (y, row) in std::io::stdin()
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| line.trim().to_string())
        .enumerate()
    {
        let mut map_row = Vec::new();
        for (x, plot) in row.chars().enumerate() {
            all_plot_points
                .entry(plot)
                .or_insert(Vec::new())
                .push(Vector2(x as isize, y as isize));
            map_row.push(usize::MAX);
        }
        plot_map.push(map_row);
    }

    let mut regions = Vec::new();

    for (plot_name, mut points) in all_plot_points.into_iter() {
        while let Some(start_point) = points.pop() {
            let mut region_points = Vec::new();
            let mut visit = vec![start_point];

            while let Some(point) = visit.pop() {
                region_points.push(point);

                let neighbor_points: Vec<Vector2> = DIRECTIONS
                    .iter()
                    .filter_map(|direction| {
                        let neighbor_point = point.add(direction);

                        if let Some(points_index) =
                            points.iter().enumerate().find_map(|(i, point)| {
                                if *point == neighbor_point {
                                    Some(i)
                                } else {
                                    None
                                }
                            })
                        {
                            Some(points.remove(points_index))
                        } else {
                            None
                        }
                    })
                    .collect();
                for neighbor_point in neighbor_points {
                    visit.push(neighbor_point);
                }
            }

            for point in region_points.iter() {
                plot_map[point.1 as usize][point.0 as usize] = regions.len();
            }

            let region = Region {
                corners: HashMap::new(),
                region_index: regions.len(),
                perimeter: Vec::new(),
                plot_name,
                points: region_points,
            };

            regions.push(region);
        }
    }

    (plot_map, regions)
}

fn part1() -> Option<isize> {
    let (map, mut regions) = read_input();

    let mut sum = 0;
    for region in regions.iter_mut() {
        region.set_perimeter(&map);
        sum += region.get_perimiter_price();
    }

    Some(sum as isize)
}

fn part2() -> Option<isize> {
    let (map, mut regions) = read_input();

    let mut sum = 0;
    for region in regions.iter_mut() {
        region.set_sides(&map);
        sum += region.get_side_price();
        println!("{region}\n");
    }

    Some(sum as isize)
}

fn main() {
    println!("--- Day 12: Garden Groups ---");
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
