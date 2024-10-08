//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.
use std::{
    collections::HashMap,
    fmt::{self, Debug, Display},
    hash::Hash,
    iter::Map
};

/// For a node `x`, when the node data is `(p, r)`, `x`'s parent is `p` and
/// `x`'s rank is `r`.
#[derive(Clone)]
pub struct NodeData<T: Eq + Hash + Clone> {
    parent: T,
    rank: usize
}

/// A collection of disjoint sets.
pub struct DisjointSets<T: Eq + Hash + Clone> {
    nodes: HashMap<T, NodeData<T>>
}

impl<T: Eq + Hash + Clone> DisjointSets<T> {
    /// Constructs an empty disjoint set.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a disjoint singleton `{v}` if `v` has not already been added.`
    pub fn add(&mut self, v: T) {
        if self.nodes.contains_key(&v) {
            return;
        }
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
    /// canonical representative. If `by_rank` is `true`, the union-by-rank
    /// optimization is used, acheiving near-linear time complexity.
    /// Otherwise, the canonical representative of `b` is chosen as the new
    /// canonical representative, which leads to log-linear complexity.
    pub fn union(&mut self, a: T, b: T, by_rank: bool) -> Option<T>
    where
        T: Display {
        let a = self.find(a)?;
        let b = self.find(b)?;
        if a != b {
            if by_rank {
                // Union-by-rank
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
            } else {
                // log::trace!("making {} the parent of {}", b, a);
                // Use `b` as new parent
                self.nodes.get_mut(&a)?.parent = b.clone();
            }
        }
        Some(a)
    }

    /// Optimizes `find` and `union` access for all nodes.
    pub fn collapse(&mut self) {
        let keys = self.nodes.keys().cloned().collect::<Vec<_>>();
        for key in keys {
            self.find(key.clone());
        }
    }
}

impl<T: Eq + Hash + Clone> Default for DisjointSets<T> {
    fn default() -> Self {
        Self {
            nodes: HashMap::default()
        }
    }
}

impl<T: Eq + Hash + Clone> Debug for DisjointSets<T>
where
    T: Debug
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, (node, data)) in self.nodes.iter().enumerate() {
            if i > 0 {
                writeln!(f)?;
            }
            write!(f, "{:?} -> {:?}", node, data.parent)?;
        }
        Ok(())
    }
}

impl<'a, T: Eq + Hash + Clone> IntoIterator for &'a DisjointSets<T> {
    type Item = (&'a T, &'a T);
    type IntoIter = Map<
        std::collections::hash_map::Iter<'a, T, NodeData<T>>,
        fn((&'a T, &'a NodeData<T>)) -> (&'a T, &'a T)
    >;

    fn into_iter(self) -> Self::IntoIter {
        self.nodes.iter().map(|(node, data)| (node, &data.parent))
    }
}
