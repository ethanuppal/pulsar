// Copyright (C) 2024 Ethan Uppal. All rights reserved.
use super::basic_block::{BasicBlock, BasicBlockCell};
use pulsar_utils::digraph::Digraph;
use std::fmt::Display;

pub struct ControlFlowGraph {
    in_graph: Digraph<BasicBlockCell, bool>,
    out_graph: Digraph<BasicBlockCell, bool>,
    entry: BasicBlockCell
}

impl ControlFlowGraph {
    pub fn new() -> Self {
        let entry = BasicBlockCell::new(BasicBlock::new());
        let mut in_graph = Digraph::new();
        let mut out_graph = Digraph::new();

        in_graph.add_node(entry.clone());
        out_graph.add_node(entry.clone());

        Self {
            in_graph,
            out_graph,
            entry
        }
    }

    pub fn entry(&self) -> BasicBlockCell {
        self.entry.clone()
    }

    pub fn new_block(&mut self) -> BasicBlockCell {
        let block = BasicBlockCell::new(BasicBlock::new());
        self.in_graph.add_node(block.clone());
        self.out_graph.add_node(block.clone());
        block
    }

    pub fn add_branch(
        &mut self, block: BasicBlockCell, condition: bool, dest: BasicBlockCell
    ) {
        self.in_graph
            .add_edge(dest.clone(), condition, block.clone());
        self.out_graph.add_edge(block, condition, dest);
    }

    pub fn size(&self) -> usize {
        self.out_graph.node_count()
    }

    pub fn blocks(&self) -> Vec<BasicBlockCell> {
        let mut result = vec![];
        self.out_graph
            .dfs(|node| result.push(node), self.entry.clone());
        result
    }
}

impl Display for ControlFlowGraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, basic_block) in self.out_graph.nodes().iter().enumerate() {
            if i > 0 {
                writeln!(f)?;
            }
            write!(f, "{}", basic_block)?;
        }
        Ok(())
    }
}
