use hashbrown::HashMap;

#[derive(Clone)]
enum OP {
    AND,
    XOR,
    OR,
}

impl OP {
    fn split(str: &str) -> (&str, Self, &str) {
        if str.contains("AND") {
            let (left, right) = str.split_once(" AND ").unwrap();
            (left, Self::AND, right)
        } else if str.contains("XOR") {
            let (left, right) = str.split_once(" XOR ").unwrap();
            (left, Self::XOR, right)
        } else {
            let (left, right) = str.split_once(" OR ").unwrap();
            (left, Self::OR, right)
        }
    }

    fn run(&self, a: Option<&usize>, b: Option<&usize>) -> Option<usize> {
        match (self, a, b) {
            (Self::AND, Some(a_value), Some(b_value)) => Some(a_value & b_value),
            (Self::OR, Some(a_value), Some(b_value)) => Some(a_value | b_value),
            (Self::XOR, Some(a_value), Some(b_value)) => Some(a_value ^ b_value),
            _ => None,
        }
    }
}

#[derive(Clone)]
struct Connection(String, OP, String, String);

impl Connection {
    fn split(str: &str) -> Connection {
        let (left_wire, op, right) = OP::split(str);

        let (right_wire, result_wire) = right.split_once(" -> ").unwrap();

        Connection(
            left_wire.to_string(),
            op,
            right_wire.to_string(),
            result_wire.to_string(),
        )
    }

    fn connect(&self, wires: &HashMap<String, usize>) -> Result<usize, ()> {
        if let Some(result) = wires.get(&self.3) {
            Ok(*result)
        } else if let Some(result) = self.1.run(wires.get(&self.0), wires.get(&self.2)) {
            Ok(result)
        } else {
            Err(())
        }
    }
}

fn read_input() -> (HashMap<String, usize>, Vec<Connection>) {
    let mut read_wires = true;
    let mut wires = HashMap::new();
    let mut connections = Vec::new();

    for line in std::io::stdin().lines().filter_map(|line| line.ok()) {
        if line.is_empty() {
            read_wires = false;
            continue;
        }
        if read_wires {
            let (name, state) = line.split_once(": ").expect("Should be able to split");
            wires.insert(name.to_string(), state.parse::<usize>().unwrap());
        } else {
            let connection = Connection::split(&line);
            connections.push(connection);
        }
    }

    (wires, connections)
}

fn get_value(part: char, wires: &HashMap<String, usize>) -> usize {
    let mut result = 0;

    for bit in (0..46).rev() {
        let part_key = format!("{part}{:0>2}", bit);
        if let Some(part_val) = wires.get(&part_key) {
            result = result << 1 | part_val;
        }
    }
    result
}

fn run(connections: &Vec<Connection>, wires: &HashMap<String, usize>) -> Result<usize, ()> {
    let mut results = wires.clone();
    loop {
        let mut did_change = false;
        let mut completed = 0;
        for connection in connections.iter() {
            if results.contains_key(&connection.3) {
                completed += 1;
            } else {
                match connection.connect(&results) {
                    Ok(result) => {
                        did_change = true;
                        completed += 1;
                        results.insert(connection.3.clone(), result);
                    }
                    Err(_) => {}
                }
            }
        }
        if completed == connections.len() {
            break;
        }
        if !did_change {
            return Err(());
        }
    }

    Ok(get_value('z', &results))
}

fn get_input(a: usize, b: usize) -> HashMap<String, usize> {
    let mut x = a;
    let mut y = b;

    let mut input = HashMap::new();

    for bit in 0..45 {
        let x_name = format!("x{:0>2}", bit);
        let y_name = format!("y{:0>2}", bit);

        input.insert(x_name, x % 2);
        input.insert(y_name, y % 2);

        x = x >> 1;
        y = y >> 1;
    }
    input
}

fn first_z_that_uses_output(output: &String, connections: &Vec<Connection>) -> Option<String> {
    // Filter connections where `c` matches either `connection.0` or `connection.2`.
    let matching_connections: Vec<&Connection> = connections
        .iter()
        .filter(|connection| &connection.0 == output || &connection.2 == output)
        .collect();

    // Check if any matching connection has an output starting with 'z'.
    if let Some(connection) = matching_connections
        .iter()
        .find(|&&conn| conn.3.starts_with('z'))
    {
        if let Ok(num) = connection.3[1..].parse::<usize>() {
            return Some(format!("z{:0>2}", num - 1));
        }
    }

    // Recursively search for `z` values in the outputs of the matching connections.
    for connection in matching_connections {
        if let Some(result) = first_z_that_uses_output(&connection.3, connections) {
            return Some(result);
        }
    }

    None
}

fn part1() -> Option<String> {
    let (mut wires, mut connections) = read_input();
    let result = run(&mut connections, &mut wires).expect("Should have a result");
    Some(result.to_string())
}

fn part2() -> Option<String> {
    let (input, mut connections) = read_input();

    // let mut swap = Vec::new();
    let x = get_value('x', &input);
    let y = get_value('y', &input);
    let expected = x + y;

    let mut nxz = connections
        .iter()
        .enumerate()
        .filter_map(|(index, connection)| {
            if connection.3.starts_with('z')
                && connection.3 != "z45"
                && !matches!(connection.1, OP::XOR)
            {
                Some(index)
            } else {
                None
            }
        })
        .collect::<Vec<usize>>();

    let mut xnz = connections
        .iter()
        .enumerate()
        .filter_map(|(index, connection)| {
            if !connection.0.starts_with(&['x', 'y'])
                && !connection.2.starts_with(&['x', 'y'])
                && !connection.3.starts_with('z')
                && matches!(connection.1, OP::XOR)
            {
                Some(index)
            } else {
                None
            }
        })
        .collect::<Vec<usize>>();

    let prev_result = run(&connections, &input).unwrap();

    for i in xnz.iter() {
        let swap = first_z_that_uses_output(&connections[*i].3, &connections).unwrap();
        let j = nxz.iter().find(|&j| connections[*j].3 == swap).unwrap();

        let temp = connections[*i].3.clone();
        connections.get_mut(*i).unwrap().3 = swap;
        connections.get_mut(*j).unwrap().3 = temp;
    }
    let x = 923232849;
    let y = 2138127;
    let input_b = get_input(x, y);
    let false_result = run(&connections, &input_b).unwrap();

    let carry_bits = "26"; // false_result ^ (x + y) => trailing zeroes

    let mut swap = Vec::new();
    for (i, connection) in connections.iter().enumerate() {
        if connection.0.ends_with(carry_bits) && connection.2.ends_with(carry_bits) {
            swap.push(i);
        }
    }
    swap.append(&mut nxz);
    swap.append(&mut xnz);

    let mut rs = swap
        .iter()
        .map(|i| connections.get(*i).unwrap().3.clone())
        .collect::<Vec<String>>();

    rs.sort();

    Some(rs.join(","))
}

fn main() {
    println!("--- Day 24: Crossed Wires ---");
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
