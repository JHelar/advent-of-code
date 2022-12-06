use std::env;
use std::fs;
use std::collections::HashSet;

fn parse_input() -> String {
    fs::read_to_string("input.txt").expect("Unable to read file!")
}

fn find_packet(packet_size: usize) -> usize {
    let content = parse_input();

    for index in 0..content.len() {
        let mut packet: HashSet<char> = HashSet::new();
        (&content[index..(index + packet_size)]).chars().for_each(| part | {
            packet.insert(part);
        });
        
        if packet.len() == packet_size {
            return index + packet_size
        }
    }

    return 0
}

fn part1() {
    let result = find_packet(4);
    println!("Packet at: {}", result);
}

fn part2() {
    let result = find_packet(14);
    println!("Packet at: {}", result);
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
