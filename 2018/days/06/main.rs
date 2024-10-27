use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Vector2(i32, i32);

impl Vector2 {
    fn from_str(str: &str) -> Self {
        let (x_str, y_str) = str.split_once(", ").unwrap();

        Self(x_str.parse().unwrap(), y_str.parse().unwrap())
    }

    fn distance_to(&self, other: Vector2) -> u16 {
        ((self.0 - other.0).abs() + (self.1 - other.1).abs()) as u16
    }

    fn get_name(&self) -> char {
        char::from_u32((65 + ((self.0 * self.1) % 25)) as u32).unwrap()
    }
}

#[derive(Debug)]
struct LocationMap {
    point_map: HashMap<Vector2, Vec<Vector2>>,
    location_map: HashMap<Vector2, Vec<Vector2>>,
    x: Vector2,
    y: Vector2,
}

impl LocationMap {
    fn create_area(locations: &Vec<Vector2>, total_point_distance: u32) -> Self {
        let x_min = locations.iter().min_by_key(|&point| point.0).unwrap().0 as i32
            - total_point_distance as i32;
        let x_max = locations.iter().max_by_key(|&point| point.0).unwrap().0 as i32
            + total_point_distance as i32;

        let y_min = locations.iter().min_by_key(|&point| point.1).unwrap().1 as i32
            - total_point_distance as i32;
        let y_max = locations.iter().max_by_key(|&point| point.1).unwrap().1 as i32
            + total_point_distance as i32;

        let mut point_map = HashMap::default();
        for x in x_min..=x_max {
            for y in y_min..=y_max {
                let point = Vector2(x, y);
                let total_distance = locations
                    .iter()
                    .map(|&location| location.distance_to(point) as u32)
                    .sum::<u32>();

                if total_distance < total_point_distance {
                    point_map.insert(point, Default::default());
                }
            }
        }

        Self {
            point_map,
            location_map: Default::default(),
            x: Vector2(x_min, x_max),
            y: Vector2(y_min, y_max),
        }
    }

    fn from_locations(locations: &Vec<Vector2>) -> Self {
        let x_min = locations.iter().min_by_key(|&point| point.0).unwrap().0 - 1;
        let x_max = locations.iter().max_by_key(|&point| point.0).unwrap().0 + 1;

        let y_min = locations.iter().min_by_key(|&point| point.1).unwrap().1 - 1;
        let y_max = locations.iter().max_by_key(|&point| point.1).unwrap().1;

        let mut location_map = HashMap::default();
        let mut point_map = HashMap::default();

        for x in x_min..=x_max {
            for y in y_min..=y_max {
                let point = Vector2(x, y);
                let closest_distance = locations
                    .iter()
                    .map(|&location| location.distance_to(point))
                    .min()
                    .unwrap();

                let point_locations: Vec<Vector2> = locations
                    .iter()
                    .filter_map(|&location| {
                        if location.distance_to(point) == closest_distance {
                            Some(location)
                        } else {
                            None
                        }
                    })
                    .collect();

                point_map.insert(point, point_locations.clone());
                if point_locations.len() == 1 {
                    let location = point_locations.iter().next().unwrap();
                    let location_points: &mut Vec<Vector2> =
                        location_map.entry(*location).or_insert(Default::default());
                    location_points.push(point);
                }
            }
        }

        Self {
            point_map,
            location_map,
            x: Vector2(x_min, x_max),
            y: Vector2(y_min, y_max),
        }
    }
}

impl Display for LocationMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in self.y.0..=self.y.1 {
            for x in self.x.0..=self.x.1 {
                let point = Vector2(x, y);
                if let Some(locations) = self.point_map.get(&point) {
                    if locations.len() > 1 {
                        write!(f, ".")?;
                    } else if locations.first().is_some_and(|&location| location == point) {
                        write!(f, "{}", locations.first().unwrap().get_name())?;
                    } else if locations.len() == 1 {
                        write!(
                            f,
                            "{}",
                            locations.first().unwrap().get_name().to_lowercase()
                        )?;
                    } else {
                        write!(f, "#")?;
                    }
                } else {
                  write!(f, " ")?;
                }
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

fn read_input() -> Vec<Vector2> {
    std::io::stdin()
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| Vector2::from_str(line.trim()))
        .collect()
}

fn part1() -> Option<u32> {
    let locations = read_input();
    let location_map = LocationMap::from_locations(&locations);

    let infinite_locations: HashSet<Vector2> = location_map
        .point_map
        .iter()
        .filter_map(|(&point, &ref locations)| {
            if locations.len() > 1 {
                None
            } else if point.0 == location_map.x.0 || point.0 == location_map.x.1 {
                Some(locations.clone())
            } else if point.1 == location_map.y.0 || point.1 == location_map.y.1 {
                Some(locations.clone())
            } else {
                None
            }
        })
        .flatten()
        .collect();

    let result = locations
        .iter()
        .filter(|&location| !infinite_locations.contains(location))
        .map(|finite_location| {
            location_map
                .location_map
                .get(finite_location)
                .unwrap()
                .len()
        })
        .max()
        .unwrap();

    Some(result as u32)
}

fn part2() -> Option<u32> {
    let locations = read_input();
    let location_map = LocationMap::create_area(&locations, 10000);
    let result = location_map.point_map.len();
    
    Some(result as u32)
}

fn main() {
    println!("--- Day 6: Chronal Coordinates ---");
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
