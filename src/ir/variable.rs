use crate::utils::id::{Gen, Id};
use std::fmt::Display;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Variable {
    id: Id
}

impl Variable {
    pub fn new() -> Self {
        Self {
            id: Gen::next("IR variable")
        }
    }
}

impl Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "i{}", self.id)
    }
}
