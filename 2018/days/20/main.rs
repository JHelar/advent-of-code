use std::{collections::VecDeque, rc::Rc};

use graph::{Edge, Graph, Node, NodeValue};
use hashbrown::{HashMap, HashSet};
use vector2::{Vector2, DOWN, LEFT, RIGHT, UP};

mod graph;
mod vector2;

fn parse_regex(
    routes: &mut VecDeque<char>,
    from_node: &Rc<Node<Vector2>>,
    nodes: &mut HashMap<String, Rc<Node<Vector2>>>,
    edges: &mut Vec<Edge<Vector2>>,
) -> Rc<Node<Vector2>> {
    let mut source_node = from_node.clone();
    while let Some(route) = routes.pop_front() {
        match route {
            'N' | 'S' | 'E' | 'W' => {
                let destination_point = match route {
                    'N' => source_node.value.add(&UP),
                    'S' => source_node.value.add(&DOWN),
                    'E' => source_node.value.add(&RIGHT),
                    'W' => source_node.value.add(&LEFT),
                    _ => panic!("Not gonna happen"),
                };

                let destination_node = Rc::new(Node::new(Rc::new(destination_point)));
                let source_dest_edge =
                    Edge::new(Rc::clone(&source_node), Rc::clone(&destination_node));
                let dest_source_edge =
                    Edge::new(Rc::clone(&destination_node), Rc::clone(&source_node));

                nodes.insert(destination_point.to_name(), destination_node.clone());
                edges.push(dest_source_edge);
                edges.push(source_dest_edge);

                source_node = destination_node;
            }
            '^' => {
                source_node = parse_regex(routes, &source_node, nodes, edges);
            }
            '(' => loop {
                parse_regex(routes, &source_node, nodes, edges);

                match routes.pop_front() {
                    Some(')') => break,
                    Some('|') => continue,
                    _ => panic!("Unknown!"),
                }
            },
            '|' => {
                routes.push_front('|');
                return source_node;
            }
            ')' => {
                routes.push_front(')');
                return source_node;
            }
            '$' => return source_node,
            _ => {
                todo!("Route not implemented {route}")
            }
        };
    }
    source_node
}

fn read_input() -> (Graph<Vector2>, Rc<Node<Vector2>>, Rc<Node<Vector2>>) {
    let mut nodes = HashMap::new();
    let mut edges = Vec::new();

    let mut regex_line = String::new();
    let _ = std::io::stdin().read_line(&mut regex_line);

    let point = Vector2(0, 0);
    let mut routes = regex_line.chars().collect::<VecDeque<char>>();
    let start_node = Rc::new(Node::new(Rc::new(point)));
    nodes.insert(point.to_name(), start_node.clone());

    let end_node = parse_regex(&mut routes, &start_node.clone(), &mut nodes, &mut edges).clone();

    (Graph::new(nodes, edges), start_node, end_node)
}

fn print_map(graph: &Graph<Vector2>, start: Vector2, with_path: &Vec<String>, with_unknown: bool) {
    let nodes: HashSet<Vector2> = graph
        .nodes
        .values()
        .map(|node| *node.value.clone())
        .collect();

    let x_start = nodes.iter().map(|vector| vector.0).min().unwrap();
    let x_end = nodes.iter().map(|vector| vector.0).max().unwrap();
    let y_start = nodes.iter().map(|vector| vector.1).min().unwrap();
    let y_end = nodes.iter().map(|vector| vector.1).max().unwrap();

    for y in y_start..=y_end {
        for x in x_start..=x_end {
            let point = Vector2(x, y);

            if !nodes.contains(&point) {
                print!("  ");
                continue;
            }

            print!("#");

            let point_up = point.add(&UP);
            match graph.edges.get(&point.to_name()) {
                Some(edges) => {
                    if edges.iter().any(|edge| *edge.destination.value == point_up) {
                        print!("-");
                    } else if with_unknown {
                        print!("?");
                    } else {
                        print!("#");
                    }
                }
                _ => {
                    if with_unknown {
                        print!("?");
                    } else {
                        print!("#");
                    }
                }
            }
        }
        print!("#");
        print!("\n");
        let point = Vector2(x_start, y);
        if !nodes.contains(&point) {
            print!(" ");
        } else {
            let left_point = point.add(&LEFT);
            match graph.edges.get(&left_point.to_name()) {
                Some(edges) => {
                    if edges
                        .iter()
                        .any(|edge| *edge.destination.value == left_point)
                    {
                        print!("|");
                    } else if with_unknown {
                        print!("?");
                    } else {
                        print!("#");
                    }
                }
                _ => {
                    if with_unknown {
                        print!("?");
                    } else {
                        print!("#");
                    }
                }
            }
        }

        for x in x_start..=x_end {
            let point = Vector2(x, y);

            if !nodes.contains(&point) {
                print!("  ");
                continue;
            }
            if point == start {
                print!("\x1b[1;42mS\x1b[0;0m");
            } else if with_path.contains(&point.to_name()) {
                print!("\x1b[1;42m \x1b[0;0m");
            } else {
                print!(".");
            }

            let point_right = point.add(&RIGHT);
            match graph.edges.get(&point.to_name()) {
                Some(edges) => {
                    if edges
                        .iter()
                        .any(|edge| *edge.destination.value == point_right)
                    {
                        print!("|");
                    } else if with_unknown {
                        print!("?");
                    } else {
                        print!("#");
                    }
                }
                _ => {
                    if with_unknown {
                        print!("?");
                    } else {
                        print!("#");
                    }
                }
            }
        }
        print!("\n");
    }
    for _ in x_start..=x_end {
        print!("##");
    }
    print!("#\n");
}

fn part1() -> Option<usize> {
    let (graph, start_node, _) = read_input();
    
    let mut best_corner = Vector2(0, 0);
    let mut best_path = Vec::new();
    graph::bfs::find_all_paths(&*start_node.value, |node: &Node<Vector2>, path| {
        if best_path.len() < path.len() {
            best_path = path;
            best_corner = *node.value.clone();
        }
    }, &graph);

    print_map(&graph, *start_node.value, &best_path, false);
    Some(best_path.len() - 1)
}

fn part2() -> Option<usize> {
    let (graph, start_node, _) = read_input();
    
    let mut rooms = HashSet::new();
    graph::bfs::find_all_paths(&*start_node.value, |node: &Node<Vector2>, path| {
        if (path.len() - 1) >= 1000 {
            rooms.insert(node.value.clone());
        }
    }, &graph);

    print_map(&graph, *start_node.value, &Vec::new(),false);
    Some(rooms.len())
}

fn main() {
    println!("--- Day 20: A Regular Map ---");
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
