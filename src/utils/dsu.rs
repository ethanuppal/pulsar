/*

void make_set(int v) {
    parent[v] = v;
    rank[v] = 0;
}

int find_set(int v) {
    if (v == parent[v])
        return v;
    return parent[v] = find_set(parent[v]);
}

void union_sets(int a, int b) {

    }
} */

use std::{collections::HashMap, hash::Hash};

pub trait NodeTrait: Eq + Hash + Clone {}

/// For a node `x`, when the node data is `(p, r)`, `x`'s parent is `p` and
/// `x`'s rank is `r`.
struct NodeData<T> {
    parent: T,
    rank: usize
}

/// A disjoint set over cheaply-cloned objects.
pub struct DisjointSets<T: NodeTrait> {
    nodes: HashMap<T, NodeData<T>>
}

impl<T: NodeTrait> DisjointSets<T> {
    /// Constructs an empty disjoint set.
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new()
        }
    }

    /// Adds a disjoint singleton `{v}`.
    pub fn add(&mut self, v: T) {
        let node_data = NodeData {
            parent: v.clone(),
            rank: 0
        };
        self.nodes.insert(v, node_data);
    }

    /// Finds the canonical representative of the set to which `v` belongs, if
    /// `v` in fact has been added via a call to [`DisjointSets::add`].
    pub fn find(&mut self, v: T) -> Option<T> {
        let p = self.nodes.get(&v)?.parent.clone();
        if v == p {
            return Some(v);
        }
        let root = self.find(p)?;
        self.nodes.get_mut(&v)?.parent = root.clone();
        Some(root)
    }

    /// Merges the sets to which `a` and `b` belong to, returning their new
    /// canonical representative.
    pub fn union(&mut self, a: T, b: T) -> Option<T> {
        let a = self.find(a)?;
        let b = self.find(b)?;
        if a != b {
            let rank_a = self.nodes.get(&a)?.rank;
            let rank_b = self.nodes.get(&b)?.rank;
            if rank_a > rank_b {
                self.nodes.get_mut(&b)?.parent = a.clone();
            } else {
                self.nodes.get_mut(&a)?.parent = b.clone();
                if rank_a == rank_b {
                    self.nodes.get_mut(&b)?.rank += 1;
                }
            }
        }
        Some(a)
    }
}
