use std::fs;
use std::collections::HashMap;

fn parse_input() -> Vec<String> {
    let contents = fs::read_to_string("./day2.input").expect("Unable to read file!");
    let lines = contents.lines();
    let mut ids = vec![];
    for line in lines {
        ids.push(line.to_string());
    }
    return ids;
}

pub fn part1() {
    let ids = parse_input();
    let mut twos = 0;
    let mut threes = 0;

    for id in ids {
        let mut occurances: HashMap<char, i32> = HashMap::new();
        for (_,char) in id.chars().enumerate() {
            if occurances.contains_key(&char) {
                occurances.insert(char, occurances[&char] + 1);
            } else {
                occurances.insert(char, 1);
            }
        }

        let mut found_threes = false;
        let mut found_twos = false;

        for (_, occurance) in &occurances {
            if !found_twos && occurance == &2 {
                found_twos = true;
                twos += 1;
            }
            if !found_threes && occurance == &3 {
                found_threes = true;
                threes += 1;
            }
            
            if found_twos && found_threes {
                break;
            }
        }
    }
    println!("Result: {} * {} = {}", twos, threes, twos * threes);
}

pub fn part2() {
    let ids = parse_input();

    for i in 0..ids.len() {
        let a_id = &ids[i];
        for j in 0..ids.len() {
            if i == j {
                continue;
            }
            let mut diffs = 0;
            let mut diff_indice = 0;
            let b_id = &ids[j];

            for k in 0..a_id.len() {
                let a_char = a_id.chars().nth(k).unwrap();
                let b_char = b_id.chars().nth(k).unwrap();

                if a_char != b_char {
                    diffs += 1;
                    diff_indice = k;
                }

                if diffs > 1 {
                    break;
                }
            }

            if diffs == 1 {
                let mut result = a_id.clone();
                result.remove(diff_indice);
                println!("Result: a_id: {}, b_id: {}, result: {}", a_id, b_id, result);
                return
            }
        }
    }
}