use crate::utils::id::{Gen, Id};

pub struct Variable {
    value: Id
}

impl Variable {
    pub fn new() -> Self {
        Self {
            value: Gen::next("IR Variable")
        }
    }
}
