use rand::{seq::SliceRandom, thread_rng};
use std::collections::{HashMap, HashSet};

fn read_input() -> HashMap<String, Vec<String>> {
    let mut wires: HashMap<String, Vec<String>> = HashMap::new();

    std::io::stdin()
        .lines()
        .filter_map(|line| line.ok())
        .for_each(|line| {
            let (name, connections_str) = line.trim().split_once(": ").unwrap();
            let mut connections: Vec<String> = connections_str
                .split(char::is_whitespace)
                .map(|connection| connection.to_string())
                .collect();

            wires
                .entry(name.to_string())
                .and_modify(|existing_connections| existing_connections.append(&mut connections))
                .or_insert(connections);
        });

    wires
}

fn cut(edges: &Vec<(String, String)>, rng: &mut impl rand::Rng) -> (usize, usize, usize) {
    let mut contracted_edges = edges
        .iter()
        .map(|edge| edge.clone())
        .clone()
        .collect::<Vec<(String, String)>>();
    let mut contracted_verticies_count = edges
        .iter()
        .flat_map(|(from, to)| [from.clone(), to.clone()])
        .collect::<HashSet<String>>()
        .len();
    let mut contracted: HashMap<String, Vec<String>> = HashMap::new();

    while contracted_verticies_count > 2 {
        let (from, to) = contracted_edges.choose(rng).unwrap();

        contracted
            .entry(from.clone())
            .and_modify(|contracts| contracts.push(to.clone()))
            .or_insert_with(|| vec![to.clone()]);

        if contracted.contains_key(to) {
            let mut contracts = contracted.get_mut(to).unwrap().clone();
            contracted.get_mut(from).unwrap().append(&mut contracts);
            contracted.remove(to);
        }

        let mut new_edges = Vec::new();
        for edge in contracted_edges.iter() {
            if edge.1 == *to {
                new_edges.push((edge.0.clone(), from.clone()));
            } else if edge.0 == *to {
                new_edges.push((from.clone(), edge.1.clone()))
            } else {
                new_edges.push(edge.clone())
            }
        }
        
        contracted_edges = new_edges.into_iter().filter(|x| x.0 != x.1).collect();
        contracted_verticies_count -= 1;
    }

    let counts: Vec<usize> = contracted
        .iter()
        .map(|(_, conntections)| conntections.len() + 1)
        .collect();

    (
        contracted_edges.len(),
        *counts.first().unwrap(),
        *counts.last().unwrap(),
    )
}

fn part1() -> Option<u32> {
    let wires = read_input();
    let edges = wires
        .iter()
        .flat_map(|(from, connections)| {
            connections
                .iter()
                .map(|connection| (from.clone(), connection.clone()))
        })
        .collect();

    let mut rng = thread_rng();
    let mut min_cut = usize::MAX;
    let mut count_1 = 0;
    let mut count_2 = 0;

    while min_cut != 3 {
        let cut_result = cut(&edges, &mut rng);
        
        min_cut = cut_result.0;
        count_1 = cut_result.1;
        count_2 = cut_result.2;
    }

    let result = count_1 * count_2;

    Some(result as u32)
}

fn part2() -> Option<u32> {
    Some(2023)
}

fn main() {
    println!("--- Day 25: Snowverload ---");
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
