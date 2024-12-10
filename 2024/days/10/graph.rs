use hashbrown::HashMap;
use std::sync::Arc;

pub trait NodeValue {
    fn to_name(&self) -> String;
}

#[derive(Debug)]
pub struct Node<T: NodeValue> {
    name: String,
    pub value: Arc<T>,
}

impl<T: NodeValue> Node<T> {
    pub fn new(value: Arc<T>) -> Node<T> {
        let name = value.to_name();

        Self { name, value }
    }
}

pub struct Edge<T: NodeValue> {
    pub origin: Arc<Node<T>>,
    pub destination: Arc<Node<T>>,
}

impl<T: NodeValue> Edge<T> {
    pub fn new(from: Arc<Node<T>>, to: Arc<Node<T>>) -> Edge<T> {
        Edge {
            origin: from,
            destination: to,
        }
    }
}

pub struct Graph<T: NodeValue> {
    pub nodes: HashMap<String, Arc<Node<T>>>,
    pub edges: HashMap<String, Vec<Edge<T>>>,
}

impl<T: NodeValue> Graph<T> {
    pub fn new(nodes: HashMap<String, Arc<Node<T>>>, edges: Vec<Edge<T>>) -> Self {
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

    pub fn get_neighbor_nodes(&self, from: &impl NodeValue) -> Vec<Arc<Node<T>>> {
        let from_name = from.to_name();
        let neighbors = self
            .edges
            .get(&from_name)
            .unwrap()
            .iter()
            .map(|edge| Arc::clone(&edge.destination))
            .collect();
        neighbors
    }

    fn get_neighbors<F>(
        &self,
        node: &String,
        mut is_edge_walkable: F,
    ) -> Vec<String> 
    where 
        F: FnMut(&Edge<T>) -> bool {
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
    use std::{collections::VecDeque, sync::Arc};

    use super::{Edge, Graph, Node, NodeValue};

    pub fn get_paths<T, F>(
        start: &Arc<T>,
        goal: &Arc<T>,
        mut is_edge_walkable: F,
        graph: &Graph<T>,
    ) -> Vec<Vec<Arc<Node<T>>>> 
    where 
        T: NodeValue,
        F: FnMut(&Edge<T>) -> bool {
        let mut paths = Vec::new();
        let mut visit = VecDeque::from([(start.to_name(), Vec::new())]);
        let goal_name = goal.to_name();

        while let Some((node_name, mut visited)) = visit.pop_back() {
            visited.push(node_name.clone());
            if node_name == goal_name {
                paths.push(visited.clone());
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
    use std::{borrow::Borrow, cmp::Ordering, collections::BTreeSet, sync::Arc, usize};

    use super::{Edge, Graph, Node, NodeValue};

    #[derive(Debug)]
    struct DistanceNode(String, Arc<usize>);

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

    pub fn get_path<T, F>(
        start: &Arc<T>,
        goal: &Arc<T>,
        mut is_edge_walkable: F,
        graph: &Graph<T>,
    ) -> Option<Vec<Arc<Node<T>>>> 
    where 
        T: NodeValue,
        F: FnMut(&Edge<T>) -> bool
    {
        let goal_name = goal.to_name();

        let mut distances: HashMap<String, Arc<usize>> = HashMap::new();
        let mut previous: HashMap<String, Option<String>> = HashMap::new();

        for (node_name, _) in graph.nodes.iter() {
            distances.insert(node_name.clone(), Arc::new(usize::MAX));
            previous.insert(node_name.clone(), None);
        }
        *Arc::make_mut(distances.get_mut(&start.to_name()).unwrap()) = 0;

        let mut visit = BTreeSet::from([DistanceNode(
            start.to_name(),
            Arc::clone(distances.get(&start.to_name()).unwrap()),
        )]);

        while let Some(distance_node) = visit.pop_first() {
            if distance_node.0 == goal_name {
                let mut current_node = goal_name;
                let mut path = vec![Arc::clone(graph.nodes.get(&current_node).unwrap())];
                while let Some(previous_node) = previous.get(&current_node).unwrap() {
                    path.push(Arc::clone(graph.nodes.get(previous_node).unwrap()));
                    current_node = previous_node.clone();
                }

                return Some(path);
            }

            let neighbors = graph.get_neighbors(&distance_node.0, &mut is_edge_walkable);
            for neighbor in neighbors {
                let new_distance = distances.get(&distance_node.0).unwrap().borrow() + 1 as usize;
                let neighbors_distance = *distances.get(&neighbor).unwrap().borrow();
                if new_distance < neighbors_distance {
                    *Arc::make_mut(distances.get_mut(&neighbor).unwrap()) = new_distance;
                    *previous.get_mut(&neighbor).unwrap() = Some(distance_node.0.clone());

                    let node = DistanceNode(
                        neighbor.to_string(),
                        Arc::clone(distances.get(&neighbor).unwrap()),
                    );
                    visit.insert(node);
                }
            }
        }
        None
    }
}
