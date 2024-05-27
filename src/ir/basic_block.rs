use super::{branch_condition::BranchCondition, Ir};
use crate::utils::{
    id::{Gen, Id},
    mutcell::MutCell
};
use std::{fmt::Display, hash::Hash};

pub struct BasicBlock {
    id: Id,
    contents: Vec<Ir>,
    branch_condition: MutCell<BranchCondition>
}

impl BasicBlock {
    pub fn new() -> Self {
        Self {
            id: Gen::next("basic block"),
            contents: vec![],
            branch_condition: MutCell::new(BranchCondition::Never)
        }
    }

    pub fn id(&self) -> Id {
        self.id
    }

    pub fn add(&mut self, ir: Ir) {
        self.contents.push(ir);
    }

    pub fn branch_condition(&self) -> BranchCondition {
        self.branch_condition.clone_out()
    }
}

impl PartialEq for BasicBlock {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

impl Eq for BasicBlock {}

impl Hash for BasicBlock {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Display for BasicBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, ".L_BB{}", self.id)?;
        for instr in &self.contents {
            writeln!(f, "  {}", instr)?;
        }
        write!(f, "  branch {}", self.branch_condition)
    }
}

impl<'a> IntoIterator for &'a BasicBlock {
    type Item = &'a Ir;
    type IntoIter = std::slice::Iter<'a, Ir>;

    fn into_iter(self) -> Self::IntoIter {
        self.contents.iter()
    }
}

pub type BasicBlockCell = MutCell<BasicBlock>;
