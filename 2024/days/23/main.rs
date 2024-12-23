use graph::Graph;
use hashbrown::HashSet;

mod graph;

#[derive(Debug, Hash, PartialEq, Eq)]
struct Computer(String);

struct Network(Graph<Computer>);

impl Network {
    fn find_all_loops(
        &self,
        start_node: &u64,
        current_node: &u64,
        visited: &mut Vec<u64>,
        length: usize,
        all_loops: &mut HashSet<Vec<u64>>,
    ) {
        if visited.len() == length {
            if start_node == current_node {
                let mut visited_clone = visited.clone();
                visited_clone.sort();
                all_loops.insert(visited_clone);
            }
            return;
        }

        let connections = self.0.get_neighbors(current_node, |_| true);
        visited.push(*current_node);

        for connection in connections {
            self.find_all_loops(start_node, &connection, visited, length, all_loops)
        }

        visited.pop();
    }
    fn find_all_sub_networks(&self, length: usize) -> HashSet<Vec<u64>> {
        let mut sub_networks = HashSet::new();
        for node in self.0.nodes.keys() {
            self.find_all_loops(node, node, &mut Vec::new(), length, &mut sub_networks);
        }

        sub_networks
    }

    fn validate_sub_network(&self, sub_network: &HashSet<u64>) -> bool {
        for node in sub_network.iter() {
            let connections = self.0.get_neighbors(node, |_| true);

            if !sub_network
                .iter()
                .all(|connection| connection == node || connections.contains(connection))
            {
                return false;
            }
        }
        return true;
    }

    fn find_loop(
        &self,
        start_node: &u64,
        current_node: &u64,
        visited: &mut HashSet<u64>,
        best_loop: &mut HashSet<u64>,
        mem: &mut HashSet<(u64, Vec<u64>)>,
    ) {
        let mut visited_vec: Vec<u64> = visited.iter().cloned().collect();
        visited_vec.sort();

        let state = (*current_node, visited_vec.clone());
        if mem.contains(&state) {
            return;
        }

        if visited.len() > 2 && start_node == current_node {
            if visited.len() > best_loop.len() {
                *best_loop = visited.clone();
            }
            mem.insert(state);
            return;
        }

        let connections = self.0.get_neighbors(current_node, |_| true);
        visited.insert(*current_node);

        for connection in connections {
            if !visited.contains(&connection) || (&connection == start_node && visited.len() > 2) {
                visited.insert(connection);
                if self.validate_sub_network(visited) {
                    self.find_loop(start_node, &connection, visited, best_loop, mem);
                }
                visited.remove(&connection);
            }
        }
        mem.insert(state);
        visited.remove(current_node);
    }

    fn find_longest_sub_network(&self) -> HashSet<u64> {
        let mut best_loop = HashSet::new();
        let mut mem = HashSet::new();
        for node in self.0.nodes.keys() {
            self.find_loop(node, node, &mut HashSet::new(), &mut best_loop, &mut mem);
        }
        best_loop
    }
}

fn read_input() -> Network {
    let mut graph = Graph::new();

    for (left, right) in std::io::stdin()
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            let (left, right) = line.split_once("-").expect("Split -");
            (left.to_string(), right.to_string())
        })
    {
        let left_computer = graph.add_node(Computer(left.to_string()));
        let right_computer = graph.add_node(Computer(right.to_string()));

        graph.add_edge(&left_computer, &right_computer, false);
    }

    Network(graph)
}

fn part1() -> Option<String> {
    let network = read_input();
    let sub_networks = network.find_all_sub_networks(3);
    let result = sub_networks
        .into_iter()
        .filter(|sub_network| {
            sub_network
                .iter()
                .any(|node_name| network.0.get_node(node_name).value.0.starts_with("t"))
        })
        .count();
    Some(result.to_string())
}

fn part2() -> Option<String> {
    let network = read_input();
    let sub_network = network.find_longest_sub_network();
    let mut computers = sub_network
        .iter()
        .map(|node_name| network.0.get_node(node_name).value.0.clone())
        .collect::<Vec<String>>();

    computers.sort();
    Some(computers.join(","))
}

fn main() {
    println!("--- Day 23: LAN Party ---");
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
