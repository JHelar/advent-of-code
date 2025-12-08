use hashbrown::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct JunctionBox {
    x: i32,
    y: i32,
    z: i32,
    connected: Option<usize>,
}

impl JunctionBox {
    fn from_str(str: &str) -> Self {
        let mut parts = str.split(",");
        let x_str = parts.next().unwrap();
        let y_str = parts.next().unwrap();
        let z_str = parts.next().unwrap();

        Self {
            x: x_str.parse().unwrap(),
            y: y_str.parse().unwrap(),
            z: z_str.parse().unwrap(),
            connected: None,
        }
    }

    fn euclidean_distance(&self, to: &Self) -> f64 {
        let dx = (self.x - to.x) as f64;
        let dy = (self.y - to.y) as f64;
        let dz = (self.z - to.z) as f64;

        dx * dx + dy * dy + dz * dz
    }
}

#[derive(Debug)]
struct Circuit {
    boxes: HashSet<usize>,
}

impl Circuit {
    fn new() -> Self {
        Self {
            boxes: HashSet::new(),
        }
    }

    fn push(&mut self, box_index: usize) {
        self.boxes.insert(box_index);
    }
}

fn read_input() -> Vec<JunctionBox> {
    std::io::stdin()
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| JunctionBox::from_str(&line.trim()))
        .collect()
}

fn get_all_connections(boxes: &Vec<JunctionBox>) -> Vec<(usize, usize, f64)> {
    let mut connections: HashMap<(usize, usize), f64> = HashMap::default();

    for a_index in 0..boxes.len() {
        for (b_index, b) in boxes.iter().enumerate() {
            if a_index == b_index {
                continue;
            }
            if connections.contains_key(&(a_index, b_index))
                || connections.contains_key(&(b_index, a_index))
            {
                continue;
            }

            let distance = boxes[a_index].euclidean_distance(b);
            connections.insert((a_index, b_index), distance);
        }
    }

    let mut connections_vec: Vec<(usize, usize, f64)> = connections
        .into_iter()
        .map(|((a_index, b_index), distance)| (a_index, b_index, distance))
        .collect();

    connections_vec.sort_by(|a, b| {
        if a.2 < b.2 {
            std::cmp::Ordering::Less
        } else if a.2 > b.2 {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    });

    connections_vec
}

fn merge_circuits(
    from_circuit_index: usize,
    to_circuit_index: usize,
    boxes: &mut Vec<JunctionBox>,
    circuits: &mut Vec<Circuit>,
) {
    let b_boxes = circuits[from_circuit_index].boxes.clone();
    circuits[from_circuit_index].boxes.clear();

    for box_index in b_boxes {
        circuits[to_circuit_index].push(box_index);
        boxes[box_index].connected = Some(to_circuit_index);
    }
}

fn part1() -> Option<usize> {
    let mut boxes = read_input();
    let mut circuits: Vec<Circuit> = Vec::new();
    let connections = get_all_connections(&boxes);

    for (a_index, b_index, _) in connections.into_iter().take(1000) {
        let a_connected = boxes[a_index].connected.clone();
        let b_connected = boxes[b_index].connected.clone();
        if a_connected.is_some() && b_connected.is_some() {
            if a_connected.unwrap() == b_connected.unwrap() {
                continue;
            }
            let a_circuit = a_connected.unwrap();
            let b_circuit = b_connected.unwrap();
            merge_circuits(b_circuit, a_circuit, &mut boxes, &mut circuits);
        } else if a_connected.is_none() && b_connected.is_none() {
            boxes[a_index].connected = Some(circuits.len());
            boxes[b_index].connected = Some(circuits.len());

            let mut circuit = Circuit::new();
            circuit.push(a_index);
            circuit.push(b_index);
            circuits.push(circuit);
        } else if let Some(circuit_id) = a_connected {
            boxes[b_index].connected = Some(circuit_id);
            circuits[circuit_id].push(b_index);
        } else if let Some(circuit_id) = b_connected {
            boxes[a_index].connected = Some(circuit_id);
            circuits[circuit_id].push(a_index);
        } else {
            panic!("Oh dear")
        }
    }

    let mut lengths = circuits
        .into_iter()
        .map(|c| c.boxes.len())
        .collect::<Vec<usize>>();
    lengths.sort_by(|a, b| b.cmp(a));
    lengths.into_iter().take(3).reduce(|a, l| a * l)
}

fn part2() -> Option<usize> {
    let mut boxes = read_input();
    let mut circuits: Vec<Circuit> = Vec::new();
    let connections = get_all_connections(&boxes);

    for (a_index, b_index, _) in connections.into_iter() {
        let a_connected = boxes[a_index].connected.clone();
        let b_connected = boxes[b_index].connected.clone();
        if a_connected.is_some() && b_connected.is_some() {
            if a_connected.unwrap() == b_connected.unwrap() {
                continue;
            }
            let a_circuit = a_connected.unwrap();
            let b_circuit = b_connected.unwrap();
            merge_circuits(b_circuit, a_circuit, &mut boxes, &mut circuits);
        } else if a_connected.is_none() && b_connected.is_none() {
            boxes[a_index].connected = Some(circuits.len());
            boxes[b_index].connected = Some(circuits.len());

            let mut circuit = Circuit::new();
            circuit.push(a_index);
            circuit.push(b_index);
            circuits.push(circuit);
        } else if let Some(circuit_id) = a_connected {
            boxes[b_index].connected = Some(circuit_id);
            circuits[circuit_id].push(b_index);
        } else if let Some(circuit_id) = b_connected {
            boxes[a_index].connected = Some(circuit_id);
            circuits[circuit_id].push(a_index);
        } else {
            panic!("Oh dear")
        }

        if boxes.iter().filter_map(|b| b.connected).count() == boxes.len() {
            let circuit_index = boxes[0].connected.unwrap();
            if boxes.iter().all(|b| b.connected.unwrap() == circuit_index) {
                return Some((boxes[a_index].x * boxes[b_index].x) as usize);
            }
        }
    }

    None
}

fn main() {
    println!("--- Day 8: Playground ---");
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
