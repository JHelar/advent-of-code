use core::time;
use std::{
    cmp::Ordering,
    collections::{BTreeMap, HashMap, HashSet},
    fmt::Display,
};

type Date = (u16, u8, u8);
type Time = (u8, u8);

#[derive(Debug, PartialEq, Eq)]
enum GuardAction {
    Begin(u32),
    Sleep,
    Wake,
}

impl GuardAction {
    fn from_str(str: &str) -> Self {
        if str.contains("wakes") {
            Self::Wake
        } else if str.contains("falls") {
            Self::Sleep
        } else {
            let guard_no = str
                .matches(char::is_numeric)
                .collect::<String>()
                .parse()
                .unwrap();
            Self::Begin(guard_no)
        }
    }
}

#[derive(Debug)]
struct Entry {
    date: Date,
    time: Time,
    action: GuardAction,
}

impl Entry {
    fn from_str(str: &str) -> Self {
        let (date_time_str, action_str) = str.split_once("] ").unwrap();
        let (date_str, time_str) = date_time_str.split_once(' ').unwrap();

        let mut date_iter = date_str.split('-');

        let year = date_iter.next().unwrap().replace("[", "").parse().unwrap();
        let month = date_iter.next().unwrap().parse().unwrap();
        let day = date_iter.next().unwrap().parse().unwrap();

        let (hour_str, minute_str) = time_str.split_once(':').unwrap();
        let hour = hour_str.parse().unwrap();
        let minute = minute_str.parse().unwrap();

        Self {
            date: (year, month, day),
            time: (hour, minute),
            action: GuardAction::from_str(action_str),
        }
    }

    fn get_time(&self) -> u64 {
        let year: u64 = self.date.0 as u64 * 1000_00_00_00_00;
        let month: u64 = self.date.1 as u64 * 10_00_00_00;
        let day: u64 = self.date.2 as u64 * 10_00_00;
        let hour: u64 = self.time.0 as u64 * 10_00;
        let minute: u64 = self.time.1 as u64 * 1;

        year + month + day + hour + minute
    }
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_time().cmp(&other.get_time())
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        self.get_time() == other.get_time() && self.action == other.action
    }
}

impl Eq for Entry {}

#[derive(Debug)]
struct SleepCalendar {
    calendar: BTreeMap<Date, (u32, Vec<Time>)>,
    guards: HashMap<u32, (u32, HashMap<Time, usize>)>,
    current_guard: Option<u32>,
    sleeping: Option<Time>,
}

impl SleepCalendar {
    fn new() -> Self {
        Self {
            calendar: Default::default(),
            guards: Default::default(),
            current_guard: None,
            sleeping: None,
        }
    }

    fn add_entry(&mut self, entry: &Entry) {
        match entry.action {
            GuardAction::Begin(guard) => self.current_guard = Some(guard),
            GuardAction::Sleep => self.sleeping = Some(entry.time),
            GuardAction::Wake => {
                let guard = self.current_guard.unwrap();
                let calendar_entry = self
                    .calendar
                    .entry(entry.date)
                    .or_insert((guard, Default::default()));

                let (from_hour, from_minute) = self.sleeping.unwrap();
                let (to_hour, to_minute) = entry.time;

                let elapsed_minutes =
                    (60 * (((24 + to_hour) - from_hour) % 24) + to_minute) - from_minute;

                let guard_entry = self.guards.entry(guard).or_insert((0, Default::default()));
                guard_entry.0 += elapsed_minutes as u32;

                for elapsed_minute in 0..elapsed_minutes {
                    let minute = (from_minute + elapsed_minute) % 60;
                    let hour = (from_hour
                        + if (from_minute + elapsed_minute) >= 60 {
                            1
                        } else {
                            0
                        })
                        % 24;
                    let time = (hour, minute);
                    let time_entry = guard_entry.1.entry(time).or_insert(0);
                    *time_entry += 1;

                    calendar_entry.1.push(time);
                }
            }
        };
    }
}

impl Display for SleepCalendar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Date   ID   Minute")?;
        writeln!(
            f,
            "               000000000011111111112222222222333333333344444444445555555555"
        )?;
        writeln!(
            f,
            "               012345678901234567890123456789012345678901234567890123456789"
        )?;

        for (date, (guard, sleep_times)) in self.calendar.iter() {
            write!(f, "{:02}-{:02}  #{:04}   ", date.1, date.2, guard)?;
            for minute in 0..60 {
                if sleep_times.contains(&(0, minute)) {
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

fn read_input() -> Vec<Entry> {
    std::io::stdin()
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| Entry::from_str(line.trim()))
        .collect()
}

fn part1() -> Option<u32> {
    let mut entries = read_input();
    entries.sort();

    let mut calendar = SleepCalendar::new();

    for entry in entries.iter() {
        calendar.add_entry(entry);
    }

    let best_guard = calendar
        .guards
        .iter()
        .max_by(|&a, &b| a.1 .0.cmp(&b.1 .0))
        .unwrap();

    let best_time = best_guard
        .1
         .1
        .iter()
        .max_by(|&a, &b| a.1.cmp(b.1))
        .unwrap();

    Some(best_guard.0 * best_time.0 .1 as u32)
}

fn part2() -> Option<u32> {
    let mut entries = read_input();
    entries.sort();

    let mut calendar = SleepCalendar::new();

    for entry in entries.iter() {
        calendar.add_entry(entry);
    }

    let best_guard = calendar
        .guards
        .iter()
        .max_by(|&a, &b| a.1 .1.values().max().cmp(&b.1 .1.values().max()))
        .unwrap();

    let ((_, minute), _) = best_guard
        .1
         .1
        .iter()
        .max_by(|&a, &b| a.1.cmp(b.1))
        .unwrap();

    Some(best_guard.0 * *minute as u32)
}

fn main() {
    println!("--- Day 4: Repose Record ---");
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
