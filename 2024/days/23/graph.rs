#![allow(dead_code)]

use hashbrown::HashMap;
use std::{
    hash::{Hash, Hasher},
    sync::{Arc, Mutex},
};

pub type NodeRef<T> = Arc<Mutex<Node<T>>>;

#[derive(Debug)]
pub struct Node<T: Hash> {
    id: u64,
    pub value: T,
}

impl<T: Hash> Node<T> {
    pub fn hash(value: &T) -> u64 {
        use std::collections::hash_map::DefaultHasher;

        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        hasher.finish()
    }

    pub fn new_ref(value: T) -> NodeRef<T> {
        Arc::new(Mutex::new(Self {
            id: Self::hash(&value),
            value,
        }))
    }

    pub fn get_ref_id(value: &NodeRef<T>) -> u64 {
        let value = value.lock().unwrap();
        value.id
    }

    pub fn get_id(value: &T) -> u64 {
        Node::hash(value)
    }
}

#[derive(Clone)]
pub struct Edge<T: Hash> {
    origin_name: u64,
    destination_name: u64,

    origin: NodeRef<T>,
    destination: NodeRef<T>,
}

impl<T: Hash> Edge<T> {
    pub fn new(from: &NodeRef<T>, to: &NodeRef<T>) -> Edge<T> {
        Edge {
            origin_name: Node::get_ref_id(from),
            origin: Arc::clone(from),
            destination_name: Node::get_ref_id(to),
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

pub struct Graph<T: Hash> {
    pub nodes: HashMap<u64, NodeRef<T>>,
    pub edges: HashMap<u64, Vec<Edge<T>>>,
}

impl<T: Hash> Graph<T> {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: T) -> u64 {
        let node_id = Node::get_id(&node);
        if self.nodes.contains_key(&node_id) {
            return node_id;
        }

        let node = Node::new_ref(node);
        self.nodes.insert(node_id, node);

        node_id
    }

    pub fn add_edge(&mut self, from_id: &u64, to_id: &u64, unidirectional: bool) {
        let edge_1 = Edge::new(self.get_node_ref(from_id), self.get_node_ref(to_id));
        self.edges
            .entry(from_id.clone())
            .or_insert(Vec::new())
            .push(edge_1);

        if unidirectional {
            return;
        }

        let edge_2 = Edge::new(self.get_node_ref(to_id), self.get_node_ref(from_id));
        self.edges
            .entry(to_id.clone())
            .or_insert(Vec::new())
            .push(edge_2);
    }

    pub fn get_neighbors<F>(&self, node: &u64, mut is_edge_walkable: F) -> Vec<u64>
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
                    Some(edge.destination_name)
                }
            })
            .collect();

        neighbors
    }

    pub fn get_node(&self, node: &u64) -> std::sync::MutexGuard<'_, Node<T>> {
        self.nodes
            .get(node)
            .expect("Should have an associated node")
            .lock()
            .unwrap()
    }

    pub fn get_node_ref(&self, node: &u64) -> &NodeRef<T> {
        self.nodes
            .get(node)
            .expect("Should have an associated node")
    }
}

pub mod bfs {
    use std::{collections::VecDeque, sync::Arc};

    use super::{Edge, Graph, Hash, Node, NodeRef};

    pub fn get_paths<T, F>(
        start: &T,
        goal: &T,
        mut is_edge_walkable: F,
        max_cost: usize,
        graph: &Graph<T>,
    ) -> Vec<Vec<NodeRef<T>>>
    where
        T: Hash,
        F: FnMut(&Edge<T>) -> bool,
    {
        let mut paths = Vec::new();
        let mut visit = VecDeque::from([(Node::get_id(start), Vec::new())]);
        let goal_name = Node::get_id(goal);

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

    use super::{Edge, Graph, Hash, Node, NodeRef};

    #[derive(Debug)]
    struct DistanceNode(u64, usize);

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
        T: Hash,
        F: FnMut(&Edge<T>) -> bool,
        FF: FnMut(&T, &T) -> usize,
    {
        let goal_name = Node::get_id(goal);
        let start_name = Node::get_id(start);

        let mut distances: HashMap<u64, usize> = HashMap::from([(start_name, 0)]);
        let mut previous: HashMap<u64, Option<u64>> = HashMap::from([(start_name, None)]);

        let mut visit = BTreeSet::from([DistanceNode(start_name, 0)]);

        while let Some(DistanceNode(current_node, current_distance)) = visit.pop_first() {
            if current_node == goal_name {
                let mut current_node = goal_name;
                let mut path: Vec<(NodeRef<T>, usize)> = vec![(
                    Arc::clone(graph.nodes.get(&current_node).unwrap()),
                    current_distance,
                )];
                while let Some(previous_node) = previous.get(&current_node).unwrap() {
                    path.push((
                        Arc::clone(graph.nodes.get(previous_node).unwrap()),
                        *distances.get(previous_node).unwrap(),
                    ));
                    current_node = previous_node.clone();
                }

                path.reverse();

                return Some(path[1..].to_vec());
            }

            let neighbors = graph.get_neighbors(&current_node, &mut is_edge_walkable);
            for neighbor in neighbors {
                let new_distance = current_distance
                    + get_cost(
                        &graph
                            .nodes
                            .get(&current_node)
                            .unwrap()
                            .lock()
                            .unwrap()
                            .value,
                        &graph.nodes.get(&neighbor).unwrap().lock().unwrap().value,
                    );
                let neighbors_distance = distances.entry(neighbor.clone()).or_insert(usize::MAX);

                if new_distance < *neighbors_distance {
                    *neighbors_distance = new_distance;
                    *previous.entry(neighbor.clone()).or_insert(None) = Some(current_node.clone());

                    let node = DistanceNode(neighbor, *neighbors_distance);
                    visit.insert(node);
                }
            }
        }
        None
    }
}
