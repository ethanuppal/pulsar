use self::{operand::Operand, variable::Variable};
use std::fmt::Display;

pub mod basic_block;
pub mod branch_condition;
pub mod control_flow_graph;
pub mod generator;
pub mod operand;
pub mod variable;

pub enum Ir {
    Add(Variable, Operand, Operand),
    Assign(Variable, Operand),
    GetParam(Variable)
}

impl Display for Ir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Add(result, lhs, rhs) => {
                write!(f, "{} = {} + {}", result, lhs, rhs)
            }
            Self::Assign(result, from) => write!(f, "{} = {}", result, from),
            Self::GetParam(result) => write!(f, "{} = <next param>", result)
        }
    }
}
