// Graph code inspired/modified from this example implementation by Nicholas D. Matsakis:
//   http://smallcultfollowing.com/babysteps/blog/2015/04/06/modeling-graphs-in-rust-using-vector-indices/

use std::collections::HashMap;
use std::fmt::{Error, Formatter};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let orbit_data: Vec<&str> = INPUT.trim().split('\n').collect();
    let mut graph = Graph {
        nodes: Vec::new(),
        edges: Vec::new(),
        node_index_by_id: HashMap::new(),
    };

    for o in orbit_data {
        let nodes: Vec<&str> = o.split(')').collect();
        let id_a = nodes.get(0).unwrap();
        let id_b = nodes.get(1).unwrap();

        let orbited_node = graph.add_node(&id_a.to_string());
        let orbiting_node = graph.add_node(&id_b.to_string());

        //        println!("{}:{} <-- {}:{}", orbiting_node, id_a, orbited_node, id_b);

        graph.add_edge(orbiting_node, orbited_node);
    }

    println!("Day 6-1: {}", day6_part1(&graph));
    println!(
        "Day 6-2: {}",
        day6_part2(&graph).expect("YOU and SAN can never reach each other :(")
    );
}

fn day6_part1(graph: &Graph) -> i32 {
    let mut orbits = 0;
    for i in 0..graph.nodes.len() {
        orbits += graph.num_reachable_nodes(i);
    }
    orbits
}

fn day6_part2(graph: &Graph) -> Option<usize> {
    // TODO: There's probably a less roundabout way of doing this.
    // find the ordered successors for YOU and SAN
    let mut you_all_successors = Vec::new();
    graph.all_successors(
        graph.node_index("YOU").expect("Where are YOU?"),
        &mut you_all_successors,
    );

    let mut san_all_successors = Vec::new();
    graph.all_successors(
        graph.node_index("SAN").expect("Where are you, SAN?!"),
        &mut san_all_successors,
    );

    // The first matching index between successors is the least common successor between
    // YOU and SAN. The number of edges is just the sum of the indices into the successors.
    for i in 0..you_all_successors.len() {
        for j in 0..san_all_successors.len() {
            if you_all_successors.get(i) == san_all_successors.get(j) {
                return Some(i + j);
            }
        }
    }
    None
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
                self.nodes.push(NodeData {
                    id: id.to_string(),
                    first_outgoing_edge: None,
                });
                self.node_index_by_id.insert(id.to_string(), index);
                index
            }
        }
    }

    fn add_edge(&mut self, source: NodeIndex, target: NodeIndex) {
        let edge_index = self.edges.len();
        let node_data = &mut self.nodes[source];
        self.edges.push(EdgeData {
            target,
            next_outgoing_edge: node_data.first_outgoing_edge,
        });
        node_data.first_outgoing_edge = Some(edge_index);
    }

    fn all_successors(&self, source: NodeIndex, vec: &mut Vec<NodeIndex>) {
        let mut iter = self.successors(source);
        while let Some(i) = iter.next() {
            if !vec.contains(&i) {
                vec.push(i);
            }
            self.all_successors(i, vec);
        }
    }

    fn successors(&self, source: NodeIndex) -> Successors {
        let first_outgoing_edge = self.nodes[source].first_outgoing_edge;
        Successors {
            graph: self,
            current_edge_index: first_outgoing_edge,
        }
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
    next_outgoing_edge: Option<EdgeIndex>,
}
