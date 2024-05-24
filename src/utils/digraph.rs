use std::{collections::HashMap, hash::Hash};

pub struct Digraph<Node: Hash + Eq, Edge> {
    adj: HashMap<Node, Vec<(Edge, Node)>>
}

impl<Node: Hash + Eq, Edge> Digraph<Node, Edge> {
    pub fn new() -> Self {
        Self {
            adj: HashMap::new()
        }
    }

    pub fn add_node(&mut self, node: Node) {
        self.adj.insert(node, vec![]);
    }

    pub fn add_edge(&mut self, u: Node, e: Edge, v: Node) {
        if let Some(out) = self.adj.get_mut(&u) {
            out.push((e, v));
        }
    }

    pub fn out_of(&self, node: Node) -> Option<&Vec<(Edge, Node)>> {
        self.adj.get(&node)
    }
}
