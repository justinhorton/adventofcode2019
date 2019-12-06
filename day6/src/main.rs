// graph code inspired/enhanced from:
//   http://smallcultfollowing.com/babysteps/blog/2015/04/06/modeling-graphs-in-rust-using-vector-indices/

use std::fmt::{Formatter, Error};
use std::collections::HashMap;
use std::iter::successors;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let orbit_data: Vec<&str> = INPUT.trim().split('\n').collect();

    let mut graph = Graph { nodes: Vec::new(), edges: Vec::new(), node_index_by_id: HashMap::new() };
    for o in orbit_data {
        let nodes: Vec<&str> = o.split(')').collect();
        let id_a = nodes.get(0).unwrap();
        let id_b = nodes.get(1).unwrap();

        let orbited_node = graph.add_node(&id_a.to_string());
        let orbiting_node = graph.add_node(&id_b.to_string());

        graph.add_edge(orbited_node, orbiting_node);
    }

    let mut orbits = 0;
    for i in 0..graph.nodes.len() {
        orbits += graph.num_reachable_nodes(i);
    }

    println!("Day 6-1: orbits: {}", orbits);
}

struct Graph {
    nodes: Vec<NodeData>,
    edges: Vec<EdgeData>,
    node_index_by_id: HashMap<String, NodeIndex>,
}

impl Graph {
    fn node_index(&self, id: &str) -> Option<NodeIndex> {
        self.node_index_by_id.get(id).copied()
    }

    fn add_node(&mut self, id: &String) -> NodeIndex {
        match self.node_index(&id) {
            Some(x) => x,
            None => {
                let index = self.nodes.len();
                self.nodes.push(NodeData { id: id.to_string(), first_outgoing_edge: None });
                self.node_index_by_id.insert(id.to_string(), index);
                index
            },
        }
    }

    fn add_edge(&mut self, source: NodeIndex, target: NodeIndex) {
        let edge_index = self.edges.len();
        let node_data = &mut self.nodes[source];
        self.edges.push(EdgeData {
            target: target,
            next_outgoing_edge: node_data.first_outgoing_edge
        });
        node_data.first_outgoing_edge = Some(edge_index);
    }

    fn successors(&self, source: NodeIndex) -> Successors {
        let first_outgoing_edge = self.nodes[source].first_outgoing_edge;
        Successors { graph: self, current_edge_index: first_outgoing_edge }
    }

    fn num_reachable_nodes(&self, source: NodeIndex) -> i32 {
        let mut iter = self.successors(source);
        let mut count = 0;
        while let Some(i) = iter.next() {
            count += 1 + self.num_reachable_nodes(i);
        }
        count
    }
}

struct Successors<'graph> {
    graph: &'graph Graph,
    current_edge_index: Option<EdgeIndex>,
}

impl<'graph> Iterator for Successors<'graph> {
    type Item = NodeIndex;

    fn next(&mut self) -> Option<NodeIndex> {
        let node_index = match self.current_edge_index {
            None => None,
            Some(edge_num) => {
                let edge = &self.graph.edges[edge_num];
                self.current_edge_index = edge.next_outgoing_edge;
                Some(edge.target)
            }
        };

        node_index

    }
}

type NodeIndex = usize;
type EdgeIndex = usize;

struct NodeData {
    id: String,
    first_outgoing_edge: Option<EdgeIndex>,
}

impl std::fmt::Display for NodeData {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", self.id)
    }
}

struct EdgeData {
    target: NodeIndex,
    next_outgoing_edge: Option<EdgeIndex>
}
