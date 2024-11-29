use hashbrown::HashMap;
use std::rc::Rc;

pub trait NodeValue {
    fn to_name(&self) -> String;
}

#[derive(Debug)]
pub struct Node<T: NodeValue> {
    name: String,
    pub value: Rc<T>,
}

impl<T: NodeValue> Node<T> {
    pub fn new(value: Rc<T>) -> Node<T> {
        let name = value.to_name();

        Self { name, value }
    }
}

pub struct Edge<T: NodeValue> {
    pub origin: Rc<Node<T>>,
    pub destination: Rc<Node<T>>,
}

impl<T: NodeValue> Edge<T> {
    pub fn new(from: Rc<Node<T>>, to: Rc<Node<T>>) -> Edge<T> {
        Edge {
            origin: from,
            destination: to,
        }
    }
}

pub struct Graph<T: NodeValue> {
    pub nodes: HashMap<String, Rc<Node<T>>>,
    pub edges: HashMap<String, Vec<Edge<T>>>,
}

impl<T: NodeValue> Graph<T> {
    pub fn new(nodes: HashMap<String, Rc<Node<T>>>, edges: Vec<Edge<T>>) -> Self {

        let mut edge_map = HashMap::new();
        for edge in edges {
            let entry = edge_map
                .entry(edge.origin.name.clone())
                .or_insert(Vec::new());
            entry.push(edge);
        }

        Self {
            nodes,
            edges: edge_map,
        }
    }

    pub fn get_neighbor_nodes(&self, from: &impl NodeValue) -> Vec<Rc<Node<T>>> {
        let from_name = from.to_name();
        let neighbors = self
            .edges
            .get(&from_name)
            .unwrap()
            .iter()
            .map(|edge| Rc::clone(&edge.destination))
            .collect();
        neighbors
    }

    fn get_neighbors(
        &self,
        node: &String,
        is_edge_walkable: &dyn Fn(&Edge<T>) -> bool,
    ) -> Vec<String> {
        let neighbors = self
            .edges
            .get(node)
            .unwrap()
            .iter()
            .filter_map(|edge| {
                if !is_edge_walkable(edge) {
                    None
                } else {
                    Some(edge.destination.name.clone())
                }
            })
            .collect();

        neighbors
    }
}

pub mod bfs {
    use std::{collections::VecDeque, rc::Rc, usize};

    use hashbrown::HashSet;

    use super::{Edge, Graph, Node, NodeValue};

    pub fn get_paths<T: NodeValue>(
        start: &impl NodeValue,
        goal: &impl NodeValue,
        max_path_len: usize,
        is_edge_walkable: &dyn Fn(&Edge<T>) -> bool,
        graph: &Graph<T>,
    ) -> Vec<Vec<Rc<Node<T>>>> {
        let mut paths = Vec::new();
        let mut visit = VecDeque::from([(start.to_name(), Vec::new())]);
        let goal_name = goal.to_name();

        while let Some((node_name, mut visited)) = visit.pop_back() {
            visited.push(node_name.clone());
            if node_name == goal_name {
                paths.push(visited.clone());
                continue;
            }

            if visited.len() >= max_path_len {
                continue;
            }

            for neighbor in graph.get_neighbors(&node_name, is_edge_walkable) {
                if visited.contains(&neighbor) {
                    continue;
                }
                visit.push_back((neighbor, visited.clone()));
            }
        }

        paths
            .into_iter()
            .filter_map(|path| {
                if path.len() == max_path_len {
                    Some(
                        path.into_iter()
                            .map(|node_name| Rc::clone(graph.nodes.get(&node_name).unwrap()))
                            .collect(),
                    )
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn find_all_paths<T, F>(
        source_node: &impl NodeValue,
        mut visiting_node: F,
        graph: &Graph<T>
    )
    where 
        T: NodeValue,
        F: FnMut(&Node<T>, Vec<String>) {
        let mut visit: VecDeque<(String, Vec<String>)> = VecDeque::from([
            (source_node.to_name(), Vec::new())
        ]);

        while let Some((node_name, mut visited)) = visit.pop_front() {
            visited.push(node_name.clone());

            visiting_node(graph.nodes.get(&node_name).unwrap(), visited.clone());

            for neighbor in graph.get_neighbors(&node_name, &|_| true) {
                if visited.contains(&neighbor) {
                    continue;
                }
                visit.push_back((neighbor, visited.clone()));
            }
        }
    }
}

pub mod dijkstra {
    use hashbrown::HashMap;
    use std::{borrow::Borrow, cmp::Ordering, collections::BTreeSet, rc::Rc, usize};

    use super::{Edge, Graph, Node, NodeValue};

    #[derive(Debug)]
    struct DistanceNode(String, Rc<usize>);

    impl Ord for DistanceNode {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            let distance_cmp = self.1.cmp(&other.1);
            match distance_cmp {
                Ordering::Equal => self.0.cmp(&other.0),
                _ => distance_cmp,
            }
        }
    }

    impl PartialOrd for DistanceNode {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(&other))
        }
    }

    impl PartialEq for DistanceNode {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }

    impl Eq for DistanceNode {}

    pub fn get_path<T: NodeValue>(
        start: &impl NodeValue,
        goal: &impl NodeValue,
        is_edge_walkable: &dyn Fn(&Edge<T>) -> bool,
        graph: &Graph<T>,
    ) -> Option<Vec<Rc<Node<T>>>> {
        let goal_name = goal.to_name();

        let mut distances: HashMap<String, Rc<usize>> = HashMap::new();
        let mut previous: HashMap<String, Option<String>> = HashMap::new();

        for (node_name, _) in graph.nodes.iter() {
            distances.insert(node_name.clone(), Rc::new(usize::MAX));
            previous.insert(node_name.clone(), None);
        }
        *Rc::make_mut(distances.get_mut(&start.to_name()).unwrap()) = 0;

        let mut visit = BTreeSet::from([DistanceNode(
            start.to_name(),
            Rc::clone(distances.get(&start.to_name()).unwrap()),
        )]);

        while let Some(distance_node) = visit.pop_first() {
            if distance_node.0 == goal_name {
                let mut current_node = goal_name;
                let mut path = vec![Rc::clone(graph.nodes.get(&current_node).unwrap())];
                while let Some(previous_node) = previous.get(&current_node).unwrap() {
                    path.push(Rc::clone(graph.nodes.get(previous_node).unwrap()));
                    current_node = previous_node.clone();
                }

                return Some(path);
            }

            let neighbors = graph.get_neighbors(&distance_node.0, is_edge_walkable);
            for neighbor in neighbors {
                let new_distance = distances.get(&distance_node.0).unwrap().borrow() + 1 as usize;
                let neighbors_distance = *distances.get(&neighbor).unwrap().borrow();
                if new_distance < neighbors_distance {
                    *Rc::make_mut(distances.get_mut(&neighbor).unwrap()) = new_distance;
                    *previous.get_mut(&neighbor).unwrap() = Some(distance_node.0.clone());

                    let node = DistanceNode(
                        neighbor.to_string(),
                        Rc::clone(distances.get(&neighbor).unwrap()),
                    );
                    visit.insert(node);
                }
            }
        }
        None
    }
}
