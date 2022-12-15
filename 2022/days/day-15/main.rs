use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;

type Coord = (i32, i32);

#[derive(Debug)]
struct Sensor {
    position: Coord,
    beacon_position: Coord,
    distance: i32,
}

impl Sensor {
    fn from_str(str: &str) -> Sensor {
        let mut splitted = str.split(':');
        let sensor_str = splitted.next().unwrap();
        let beacon_str = splitted.next().unwrap();

        let mut sensor_positions = sensor_str[10..]
            .split(", ")
            .map(|x| x[2..].parse::<i32>().unwrap());
        let sensor_x = sensor_positions.next().unwrap();
        let sensor_y = sensor_positions.next().unwrap();
        let sensor_position = (sensor_x, sensor_y);

        let mut beacon_positions = beacon_str[22..]
            .split(", ")
            .map(|x| x[2..].parse::<i32>().unwrap());
        let beacon_x = beacon_positions.next().unwrap();
        let beacon_y = beacon_positions.next().unwrap();
        let beacon_position = (beacon_x, beacon_y);

        let distance = manhattan_distance(sensor_position, beacon_position);

        Sensor {
            position: sensor_position,
            beacon_position,
            distance,
        }
    }
}

fn manhattan_distance((x1, y1): Coord, (x2, y2): Coord) -> i32 {
    (x1 - x2).abs() + (y1 - y2).abs()
}

fn parse_input() -> String {
    fs::read_to_string("input.txt").expect("Unable to read file!")
}

fn parse_sensors() -> HashMap<Coord, Sensor> {
    let content = parse_input();
    let mut sensors: HashMap<Coord, Sensor> = Default::default();
    content
        .lines()
        .map(|line| Sensor::from_str(line.trim()))
        .for_each(|sensor| {
            sensors.insert(sensor.position, sensor);
        });
    sensors
}

fn part1() {
    let sensors = parse_sensors();
    let y_pos = 2000000;

    let mut not_beacons: HashSet<Coord> = Default::default();

    // For each sensor see if it covers the y_pos
    for sensor in sensors.values() {
        let distance = (sensor.position.1 - y_pos).abs();
        if distance <= sensor.distance {
            let start_x = sensor.position.0 - (sensor.distance - distance);
            let end_x = sensor.position.0 + (sensor.distance - distance);
            for x in start_x..end_x {
                let pos = (x, y_pos);
                if !sensors.contains_key(&pos) {
                    not_beacons.insert(pos);
                }
            }
        }
    }

    println!("Result: {}", not_beacons.len());
}

fn check_pos((x, y): Coord, max: i32, sensors: &HashMap<Coord, Sensor>) -> bool {
    if x < 0 {
        return false;
    }
    if y < 0 {
        return false;
    }
    if x > max {
        return false;
    }
    if y > max {
        return false;
    }
    let cel = (x, y);
    for sensor in sensors.values() {
        let d_sensor = manhattan_distance(sensor.position, cel);
        let d_beacon = manhattan_distance(sensor.beacon_position, cel);
        if d_sensor <= sensor.distance || d_beacon == 0 {
            return false;
        }
    }

    println!("Result: {}", (x as i64 * 4000000) + y as i64);
    return true;
}

fn part2() {
    let sensors = parse_sensors();
    let max = 4000000;

    for sensor in sensors.values() {
        let x = sensor.position.0;
        let y = sensor.position.1;
        let d = sensor.distance + 1;

        for i in 0..d {
            if check_pos((x + i, y - d + i), max, &sensors) {
                return;
            }
            if check_pos((x + d - i, y + i), max, &sensors) {
                return;
            }
            if check_pos((x - i, y + d - i), max, &sensors) {
                return;
            }
            if check_pos((x - d + i, y - i), max, &sensors) {
                return;
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let part = &args[1];

    if part == "1" {
        part1();
    } else if part == "2" {
        part2();
    }
}
