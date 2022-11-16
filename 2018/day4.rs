use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
enum Order {
    Wake,
    Sleep,
    Begin(String),
}

#[derive(Debug, Eq)]
struct DateTime(i32, i32, i32, i32, i32);

impl Ord for DateTime {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.0 > other.0 {
            return Ordering::Greater;
        } else if self.0 < other.0 {
            return Ordering::Less;
        }

        if self.1 > other.1 {
            return Ordering::Greater;
        } else if self.1 < other.1 {
            return Ordering::Less;
        }

        if self.2 > other.2 {
            return Ordering::Greater;
        } else if self.2 < other.2 {
            return Ordering::Less;
        }

        if self.3 > other.3 {
            return Ordering::Greater;
        } else if self.3 < other.3 {
            return Ordering::Less;
        }

        if self.4 > other.4 {
            return Ordering::Greater;
        } else if self.4 < other.4 {
            return Ordering::Less;
        }

        return Ordering::Equal;
    }
}

impl PartialOrd for DateTime {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for DateTime {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

#[derive(Debug)]
struct Record(DateTime, Order);

fn parse_input() -> Vec<Record> {
    let guard_id_regex = Regex::new(r"#(\d+)").unwrap();
    let contents = fs::read_to_string("./day4.input").expect("Unable to read file!");
    let lines = contents.lines();
    let mut records: Vec<Record> = vec![];

    for line in lines {
        let parts = line.split(']').map(|x| x.trim()).collect::<Vec<&str>>();

        let date_time_str = parts[0][1..]
            .split(char::is_whitespace)
            .collect::<Vec<&str>>();
        let date_parts = date_time_str[0]
            .split('-')
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let time_parts = date_time_str[1]
            .split(':')
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let order = parts[1];
        if guard_id_regex.is_match(order) {
            let id = guard_id_regex
                .captures(&order)
                .unwrap()
                .get(0)
                .unwrap()
                .as_str();

            records.push(Record(
                DateTime(
                    date_parts[0],
                    date_parts[1],
                    date_parts[2],
                    time_parts[0],
                    time_parts[1],
                ),
                Order::Begin(id.to_string()),
            ));
        } else if order.contains("sleep") {
            records.push(Record(
                DateTime(
                    date_parts[0],
                    date_parts[1],
                    date_parts[2],
                    time_parts[0],
                    time_parts[1],
                ),
                Order::Sleep,
            ));
        } else {
            records.push(Record(
                DateTime(
                    date_parts[0],
                    date_parts[1],
                    date_parts[2],
                    time_parts[0],
                    time_parts[1],
                ),
                Order::Wake,
            ));
        }
    }

    records.sort_by(|Record(a_date_time, _), Record(b_date_time, _)| a_date_time.cmp(b_date_time));

    return records;
}

pub fn part1() {
    let records = parse_input();
    let mut schedules: HashMap<String, Vec<(DateTime, DateTime, i32)>> = HashMap::new();

    let mut current_guard_id: String = String::new();
    let mut sleep_date_time: DateTime = DateTime(0, 0, 0, 0, 0);

    for Record(date_time, order) in records {
        match order {
            Order::Begin(guard_id) => {
                if !schedules.contains_key(&guard_id) {
                    schedules.insert(guard_id.clone(), vec![]);
                }
                current_guard_id = guard_id;
            }
            Order::Sleep => sleep_date_time = date_time,
            Order::Wake => {
                let sleep_minute = sleep_date_time.4.clone();
                let wake_minute = date_time.4.clone();
                let sleep = wake_minute - sleep_minute;

                schedules.get_mut(&current_guard_id.clone()).unwrap().push((
                    DateTime(
                        sleep_date_time.0,
                        sleep_date_time.1,
                        sleep_date_time.2,
                        sleep_date_time.3,
                        sleep_date_time.4,
                    ),
                    date_time,
                    sleep,
                ));
            }
        }
    }

    let mut best_sleep_time = 0;
    let mut best_guard = String::new();
    for (guard, schedule) in &schedules {
        let mut sleep_time = 0;
        for (_sleep, _wake, time) in schedule {
            sleep_time += time;
        }

        if sleep_time > best_sleep_time {
            best_sleep_time = sleep_time;
            best_guard = guard.clone();
        }
    }

    let mut sleep_times: HashMap<i32, i32> = HashMap::new();
    let mut best_time = 0;
    let mut best_hour = 0;
    let schedule = schedules.get_mut(&best_guard).unwrap();
    for (sleep, wake, _) in schedule {
        for slept in sleep.4..wake.4 {
            let value = sleep_times.get(&slept).unwrap_or(&0);
            let new_value = *value + 1;
            sleep_times.insert(slept, new_value);

            if new_value > best_time {
                best_time = new_value;
                best_hour = slept;
            }
        }
    }

    println!("{} => {}", best_guard, best_hour);
}

pub fn part2() {
    let records = parse_input();
    let mut schedules: HashMap<String, HashMap<i32, i32>> = HashMap::new();

    let mut current_guard_id: String = String::new();
    let mut sleep_date_time: DateTime = DateTime(0, 0, 0, 0, 0);
    let mut best_sleep_time = 0;
    let mut best_sleep_minute = 0;
    let mut best_guard = String::new();

    for Record(date_time, order) in records {
        match order {
            Order::Begin(guard_id) => {
                if !schedules.contains_key(&guard_id) {
                    schedules.insert(guard_id.clone(), HashMap::new());
                }
                current_guard_id = guard_id;
            }
            Order::Sleep => sleep_date_time = date_time,
            Order::Wake => {
                let sleep_minute = sleep_date_time.4.clone();
                let wake_minute = date_time.4.clone();
                let sleep_times = schedules.get_mut(&current_guard_id).unwrap();

                for slept in sleep_minute..wake_minute {
                    let value = sleep_times.get(&slept).unwrap_or(&0);
                    let new_value = *value + 1;
                    sleep_times.insert(slept, new_value);

                    if new_value > best_sleep_time {
                        best_sleep_time = new_value;
                        best_sleep_minute = slept;
                        best_guard = current_guard_id.clone();
                    }
                }
            }
        }
    }

    println!("{} => {}", best_guard, best_sleep_minute);
}
