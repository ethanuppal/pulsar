use self::{label::LabelName, operand::Operand, variable::Variable};
use std::fmt::Display;

pub mod basic_block;
pub mod branch_condition;
pub mod control_flow_graph;
pub mod generator;
pub mod label;
pub mod operand;
pub mod variable;

pub enum Ir {
    Add(Variable, Operand, Operand),
    Mul(Variable, Operand, Operand),
    Assign(Variable, Operand),
    GetParam(Variable),
    Return(Option<Operand>),
    LocalAlloc(Variable, usize),
    Store {
        result: Variable,
        value: Operand,
        index: usize
    },
    Map {
        result: Variable,
        parallel_factor: usize,
        f: LabelName,
        input: Operand
    },
    Call(Option<Variable>, LabelName, Vec<Operand>)
}

impl Display for Ir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Add(result, lhs, rhs) => {
                write!(f, "{} = {} + {}", result, lhs, rhs)
            }
            Self::Mul(result, lhs, rhs) => {
                write!(f, "{} = {} * {}", result, lhs, rhs)
            }
            Self::Assign(result, from) => write!(f, "{} = {}", result, from),
            Self::GetParam(result) => write!(f, "{} = <next param>", result),
            Self::Return(value_opt) => write!(
                f,
                "ret{}",
                if let Some(value) = value_opt {
                    format!(" {}", value)
                } else {
                    "".into()
                }
            ),
            Self::LocalAlloc(result, size) => {
                write!(f, "{} = <{} bytes>", result, size)
            }
            Self::Store {
                result,
                value,
                index
            } => {
                write!(f, "{}[{}] = {}", result, index, value)
            }
            Self::Map {
                result,
                parallel_factor,
                f: func,
                input
            } => {
                write!(
                    f,
                    "{} = map<{}>({}, {})",
                    result, parallel_factor, func, input
                )
            }
            Self::Call(result_opt, name, args) => {
                write!(
                    f,
                    "{}{}({})",
                    if let Some(result) = result_opt {
                        format!("{} = ", result)
                    } else {
                        "".into()
                    },
                    name,
                    args.iter()
                        .map(|arg| arg.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
        }
    }
}
