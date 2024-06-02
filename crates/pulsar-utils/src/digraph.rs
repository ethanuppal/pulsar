// Copyright (C) 2024 Ethan Uppal. All rights reserved.
use std::{
    collections::{HashMap, HashSet},
    hash::Hash
};

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

    pub fn nodes(&self) -> Vec<&Node> {
        self.adj.keys().collect()
    }

    pub fn node_count(&self) -> usize {
        self.adj.len()
    }
}

impl<Node: Hash + Eq + Clone, Edge> Digraph<Node, Edge> {
    /// Conducts a depth-first search (DFS) starting from `start`, calling `f`
    /// on each node encountered in DFS order. This function performs multiple
    /// `clone()`s of the nodes, which is still performant when nodes are
    /// smart pointers such as `Rc`.
    ///
    /// Requires: `start` is in the graph.
    pub fn dfs<F>(&self, mut f: F, start: Node)
    where
        F: FnMut(Node) {
        assert!(self.adj.contains_key(&start));

        let mut visited = HashSet::new();
        let mut stack = vec![];

        stack.push(start);
        while !stack.is_empty() {
            let node = stack.pop().unwrap();
            visited.insert(node.clone());
            f(node.clone());
            for (_, next) in self.out_of(node).unwrap() {
                stack.push(next.clone());
            }
        }
    }
}
