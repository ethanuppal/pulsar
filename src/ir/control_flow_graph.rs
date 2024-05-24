use super::basic_block::BasicBlockCell;
use crate::utils::digraph::Digraph;

pub struct ControlFlowGraph {
    in_graph: Digraph<BasicBlockCell, bool>,
    out_graph: Digraph<BasicBlockCell, bool>
}

impl ControlFlowGraph {}
