use std::collections::HashMap;
use std::env;
use std::fs;

type Cave = HashMap<String, Valve>;

#[derive(Debug, Clone)]
struct Valve {
    links: Vec<String>,
    rate: i32,
    name: String,
}

impl Valve {
    fn from_str(str: &str) -> Valve {
        let mut str_split = str.split(';');
        let name_rate_str = str_split.next().unwrap();
        let links_str = str_split.next().unwrap();

        let name = name_rate_str[6..8].to_string();
        let rate = name_rate_str[23..].parse::<i32>().unwrap();

        let mut links_str_split = links_str.split(',').map(|x| x.trim());
        let first_link_string = links_str_split
            .next()
            .unwrap()
            .chars()
            .rev()
            .collect::<String>();
        let first_link = &first_link_string[0..2].chars().rev().collect::<String>();
        let mut links: Vec<String> = vec![first_link.clone()];

        links_str_split.for_each(|link| {
            links.push(link.to_string());
        });

        Valve { links, rate, name }
    }
}

fn parse_input() -> String {
    fs::read_to_string("input.txt").expect("Unable to read file!")
}

fn parse_cave() -> Cave {
    let content = parse_input();
    let mut valves: Cave = Default::default();
    content.lines().map(Valve::from_str).for_each(|valve| {
        valves.insert(valve.name.clone(), valve);
    });
    valves
}

fn f(
    player: String,
    cave: &Cave,
    time: i32,
    other_players: i32,
    opened_valves: &Vec<String>,
    mem: &mut HashMap<String, i32>,
) -> i32 {
    if time == 0 {
        if other_players > 0 {
            return f(
                "AA".to_string(),
                cave,
                26,
                other_players - 1,
                opened_valves,
                mem,
            );
        }
        return 0;
    }

    let key = format!(
        "{}-{}-{}-{}", opened_valves.join(","), player, time, other_players
    );
    if mem.contains_key(&key) {
        return *mem.get(&key).unwrap();
    }

    let mut ans = 0;
    let valve = cave.get(&player).unwrap();
    if !opened_valves.contains(&player) && valve.rate > 0 {
        let mut new_opened_valves = Vec::from_iter(opened_valves.iter().map(|v| v.clone()));
        new_opened_valves.push(player.clone());
        ans = ans.max(
            (time - 1) * valve.rate
                + f(
                    player.clone(),
                    cave,
                    time - 1,
                    other_players,
                    &new_opened_valves,
                    mem,
                ),
        );
    }
    for link in valve.links.iter() {
        ans = ans.max(f(
            link.clone(),
            cave,
            time - 1,
            other_players,
            opened_valves,
            mem,
        ));
    }
    mem.insert(key, ans);
    ans
}

fn part1() {
    let cave = parse_cave();
    let mut mem: HashMap<String, i32> = Default::default();
    let opened_valves: Vec<String> = Vec::new();
    let ans = f("AA".to_string(), &cave, 30, 0, &opened_valves, &mut mem);
    println!("Result: {:?}", ans);
}

fn part2() {
    let cave = parse_cave();
    let mut mem: HashMap<String, i32> = Default::default();
    let opened_valves: Vec<String> = Vec::new();
    let ans = f("AA".to_string(), &cave, 26, 1, &opened_valves, &mut mem);
    println!("Result: {:?}", ans);
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
