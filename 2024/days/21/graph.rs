#![allow(dead_code)]

use hashbrown::HashMap;
use std::sync::{Arc, Mutex};

pub type NodeRef<T> = Arc<Mutex<Node<T>>>;

pub trait NodeValue {
    fn to_name(&self) -> String;
}

#[derive(Debug)]
pub struct Node<T: NodeValue> {
    name: String,
    pub value: T,
}

impl<T: NodeValue> Node<T> {
    pub fn new_ref(value: T) -> NodeRef<T> {
        let name = value.to_name();

        Arc::new(Mutex::new(Self { name, value }))
    }

    pub fn get_name(value: &NodeRef<T>) -> String {
        let value = value.lock().unwrap();
        value.name.clone()
    }
}

#[derive(Clone)]
pub struct Edge<T: NodeValue> {
    origin_name: String,
    destination_name: String,

    origin: NodeRef<T>,
    destination: NodeRef<T>,
}

impl<T: NodeValue> Edge<T> {
    pub fn new(from: &NodeRef<T>, to: &NodeRef<T>) -> Edge<T> {
        Edge {
            origin_name: Node::get_name(from),
            origin: Arc::clone(from),
            destination_name: Node::get_name(to),
            destination: Arc::clone(to),
        }
    }

    pub fn get_origin(&self) -> std::sync::MutexGuard<'_, Node<T>> {
        self.origin.lock().unwrap()
    }

    pub fn get_destination(&self) -> std::sync::MutexGuard<'_, Node<T>> {
        self.destination.lock().unwrap()
    }
}

pub struct Graph<T: NodeValue> {
    pub nodes: HashMap<String, NodeRef<T>>,
    pub edges: HashMap<String, Vec<Edge<T>>>,
}

impl<T: NodeValue> Graph<T> {
    pub fn new(nodes: HashMap<String, NodeRef<T>>, edges: Vec<Edge<T>>) -> Self {
        let mut edge_map = HashMap::new();
        for edge in edges {
            let entry = edge_map
                .entry(edge.origin_name.clone())
                .or_insert(Vec::new());
            entry.push(edge);
        }

        Self {
            nodes,
            edges: edge_map,
        }
    }

    fn get_neighbors<F>(&self, node: &String, mut is_edge_walkable: F) -> Vec<String>
    where
        F: FnMut(&Edge<T>) -> bool,
    {
        let neighbors = self
            .edges
            .get(node)
            .unwrap()
            .iter()
            .filter_map(|edge| {
                if !is_edge_walkable(edge) {
                    None
                } else {
                    Some(edge.destination_name.clone())
                }
            })
            .collect();

        neighbors
    }
}

pub mod bfs {
    use std::{collections::VecDeque, sync::Arc};

    use super::{Edge, Graph, NodeRef, NodeValue};

    pub fn get_paths<T, F>(
        start: &T,
        goal: &T,
        mut is_edge_walkable: F,
        max_cost: usize,
        graph: &Graph<T>,
    ) -> Vec<Vec<NodeRef<T>>>
    where
        T: NodeValue,
        F: FnMut(&Edge<T>) -> bool,
    {
        let mut paths = Vec::new();
        let mut visit = VecDeque::from([(start.to_name(), Vec::new())]);
        let goal_name = goal.to_name();

        while let Some((node_name, mut visited)) = visit.pop_back() {
            visited.push(node_name.clone());
            if node_name == goal_name {
                paths.push(visited[1..].to_vec());
                continue;
            }

            if visited.len() >= max_cost {
                continue;
            }

            for neighbor in graph.get_neighbors(&node_name, &mut is_edge_walkable) {
                if visited.contains(&neighbor) {
                    continue;
                }
                visit.push_back((neighbor, visited.clone()));
            }
        }

        paths
            .into_iter()
            .map(|path| {
                path.into_iter()
                    .map(|node_name| Arc::clone(graph.nodes.get(&node_name).unwrap()))
                    .collect()
            })
            .collect()
    }
}

pub mod dijkstra {
    use hashbrown::HashMap;
    use std::{cmp::Ordering, collections::BTreeSet, sync::Arc};

    use super::{Edge, Graph, NodeRef, NodeValue};

    #[derive(Debug)]
    struct DistanceNode(String, usize);

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

    pub fn get_path<T, F, FF>(
        start: &T,
        goal: &T,
        mut is_edge_walkable: F,
        mut get_cost: FF,
        graph: &Graph<T>,
    ) -> Option<Vec<(NodeRef<T>, usize)>>
    where
        T: NodeValue,
        F: FnMut(&Edge<T>) -> bool,
        FF: FnMut(&T, &T) -> usize,
    {
        let goal_name = goal.to_name();

        let mut distances: HashMap<String, usize> = HashMap::from([(start.to_name(), 0)]);
        let mut previous: HashMap<String, Option<String>> =
            HashMap::from([(start.to_name(), None)]);

        let mut visit = BTreeSet::from([DistanceNode(start.to_name(), 0)]);

        while let Some(DistanceNode(current_node, current_distance)) = visit.pop_first() {
            if current_node == goal_name {
                let mut current_node = goal_name;
                let mut path: Vec<(NodeRef<T>, usize)> = vec![(Arc::clone(graph.nodes.get(&current_node).unwrap()), current_distance)];
                while let Some(previous_node) = previous.get(&current_node).unwrap() {
                    path.push((Arc::clone(graph.nodes.get(previous_node).unwrap()), *distances.get(previous_node).unwrap()));
                    current_node = previous_node.clone();
                }

                path.reverse();

                return Some(path[1..].to_vec());
            }

            let neighbors = graph.get_neighbors(&current_node, &mut is_edge_walkable);
            for neighbor in neighbors {
                let new_distance = current_distance + get_cost(&graph.nodes.get(&current_node).unwrap().lock().unwrap().value, &graph.nodes.get(&neighbor).unwrap().lock().unwrap().value);
                let neighbors_distance = distances.entry(neighbor.clone()).or_insert(usize::MAX);

                if new_distance < *neighbors_distance {
                    *neighbors_distance = new_distance;
                    *previous.entry(neighbor.clone()).or_insert(None) = Some(current_node.clone());

                    let node = DistanceNode(neighbor.to_string(), *neighbors_distance);
                    visit.insert(node);
                }
            }
        }
        None
    }
    pub fn get_path_lengths<T, F>(
        start: &T,
        goal: &T,
        mut is_edge_walkable: F,
        max_cost: usize,
        graph: &Graph<T>,
    ) -> Vec<usize>
    where
        T: NodeValue,
        F: FnMut(&Edge<T>) -> bool,
    {
        let goal_name = goal.to_name();

        let mut distances: HashMap<String, usize> = HashMap::from([(start.to_name(), 0)]);
        let mut previous: HashMap<String, Option<String>> =
            HashMap::from([(start.to_name(), None)]);

        let mut visit = BTreeSet::from([DistanceNode(start.to_name(), 0)]);
        let mut lengths = Vec::new();

        while let Some(DistanceNode(current_node, current_distance)) = visit.pop_first() {
            if current_node == goal_name {
                lengths.push(current_distance);
                continue;
            }
            if current_distance > max_cost {
                continue;
            }

            let neighbors = graph.get_neighbors(&current_node, &mut is_edge_walkable);
            for neighbor in neighbors {
                let new_distance = current_distance + 1;
                let neighbors_distance = distances.entry(neighbor.clone()).or_insert(usize::MAX);

                if new_distance < *neighbors_distance {
                    *neighbors_distance = new_distance;
                    *previous.entry(neighbor.clone()).or_insert(None) = Some(current_node.clone());

                    let node = DistanceNode(neighbor.to_string(), *neighbors_distance);
                    visit.insert(node);
                }
            }
        }
        lengths
    }
}
