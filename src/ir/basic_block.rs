use super::{branch_condition::BranchCondition, Ir};
use crate::utils::{id::Id, mutcell::MutCell};
use std::hash::Hash;

pub struct BasicBlock {
    id: Id,
    contents: Vec<Ir>,
    branch_condition: BranchCondition
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

pub type BasicBlockCell = MutCell<BasicBlock>;
