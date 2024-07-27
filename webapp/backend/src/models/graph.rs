use sqlx::FromRow;
use std::collections::HashMap;
use std::{cmp::Ordering, collections::BinaryHeap};

#[derive(FromRow, Clone, Debug)]
pub struct Node {
    pub id: i32,
    pub x: i32,
    pub y: i32,
}

#[derive(FromRow, Clone, Debug)]
pub struct Edge {
    pub node_a_id: i32,
    pub node_b_id: i32,
    pub weight: i32,
}

#[derive(Clone, PartialEq, Eq)]
pub struct State<T> {
    pub id: i32,
    pub priority: T,
}

impl<T> Ord for State<T>
where
    T: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority).reverse()
    }
}

impl<T> PartialOrd for State<T>
where
    T: Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
pub struct Graph {
    pub nodes: HashMap<i32, Node>,
    pub edges: HashMap<i32, Vec<Edge>>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.id, node);
    }

    pub fn add_edge(&mut self, edge: Edge) {
        self.edges
            .entry(edge.node_a_id)
            .or_default()
            .push(edge.clone());

        let reverse_edge = Edge {
            node_a_id: edge.node_b_id,
            node_b_id: edge.node_a_id,
            weight: edge.weight,
        };
        self.edges
            .entry(reverse_edge.node_a_id)
            .or_default()
            .push(reverse_edge);
    }

    pub fn shortest_path(&self, from_node_id: i32, to_node_id: i32) -> i32 {
        let mut distances = BinaryHeap::new();
        distances.push(State {id: from_node_id, priority: 0});
        let mut is_confirmed = HashMap::new();
        is_confirmed.insert(from_node_id, true);

        while let Some(state) = distances.pop() {
            if state.id == to_node_id {
                return state.priority;
            }
            if let Some(val) = is_confirmed.get(state.id) {
                continue;
            }
            is_confirmed.insert(state.id, true);
            if let Some(edges) = self.edges.get(state.id) {
                for edge in edges {
                    if let Some(val) = is_confirmed.get(edge.node_b_id) {
                    } else {
                        distances.push(State {id: edge.node_b_id, priority: state.priority + edge.weight});
                    }
                }
            }
        }
        -1
    }
}
