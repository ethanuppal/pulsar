use crate::utils::{id::Id, mutcell::MutCell};
use lazy_static::lazy_static;
use std::{fmt::Display, hash::Hash};

lazy_static! {
    pub static ref UNIT_TYPE_CELL: TypeCell = TypeCell::new(Type::Unit);
    pub static ref INT64_TYPE_CELL: TypeCell = TypeCell::new(Type::Int64);
}

pub const ARRAY_TYPE_UNKNOWN_SIZE: isize = -1;

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub enum Type {
    Unknown,
    Unit,
    Var(Id),
    Name(String),
    Int64,

    /// A negative size indicates that the size is not yet known.
    Array(TypeCell, isize),

    Function {
        is_pure: bool,
        args: Vec<Type>,
        ret: Box<Type>
    }
}

impl Type {
    pub fn unwrap(self) -> Self {
        match self {
            Self::Unknown => panic!("Type::unwrap failed"),
            other => other
        }
    }

    pub fn is_known(self) -> bool {
        match self {
            Self::Unknown => false,
            _ => true
        }
    }

    pub fn make_unknown() -> TypeCell {
        TypeCell::new(Self::Unknown)
    }

    pub fn int64_singleton() -> TypeCell {
        INT64_TYPE_CELL.to_owned()
    }

    pub fn unit_singleton() -> TypeCell {
        UNIT_TYPE_CELL.to_owned()
    }

    /// The number of bytes to store one instance of a value of the current
    /// type.
    pub fn size(&self) -> usize {
        match &self {
            Type::Unknown => panic!("Type::Unknown does not have a size"),
            Type::Unit => 0,
            Type::Var(_) => {
                panic!("Type::Var should have been resolved by type inference")
            }
            Type::Name(_) => todo!("Need to figure out user-defined types"),
            Type::Int64 => 8,
            Type::Array(element_type, element_count) => {
                element_type.as_ref().size() * (*element_count as usize)
            }
            Type::Function {
                is_pure: _,
                args: _,
                ret: _
            } => 8
        }
    }

    pub fn as_array_type(&self) -> (TypeCell, isize) {
        match &self {
            Self::Array(element_type, size) => (element_type.clone(), *size),
            _ => panic!(
                "{}",
                format!(
                    "Type::as_array_type called on non-array type `{}`",
                    &self
                )
            )
        }
    }

    pub fn mangle(&self) -> String {
        match &self {
            Type::Unknown | Type::Var(_) => panic!(),
            Type::Unit => "u".into(),
            Type::Name(name) => format!("{}{}", name.len(), name),
            Type::Int64 => "q".into(),
            Type::Array(element_type, count) => {
                format!("A{}{}", count, element_type)
            }
            Type::Function {
                is_pure: _,
                args: _,
                ret: _
            } => todo!()
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown => write!(f, "?"),
            Self::Unit => write!(f, "Unit"),
            Self::Var(var) => write!(f, "'t{}", var),
            Self::Name(name) => write!(f, "{}", name),
            Self::Int64 => write!(f, "Int64"),
            Self::Array(tcell, size) => write!(
                f,
                "{}[{}]",
                tcell,
                if *size == ARRAY_TYPE_UNKNOWN_SIZE {
                    "?".into()
                } else {
                    size.to_string()
                }
            ),
            Self::Function { is_pure, args, ret } => write!(
                f,
                "{}({}) -> {}",
                if *is_pure { "pure " } else { "" },
                args.iter()
                    .map(|ty| ty.to_string())
                    .collect::<Vec<_>>()
                    .join(", "),
                ret,
            )
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum StmtTermination {
    Terminal,
    Nonterminal
}

impl Display for StmtTermination {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Terminal => "Terminal",
            Self::Nonterminal => "Nonterminal"
        }
        .fmt(f)
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct StmtType {
    pub termination: StmtTermination,
    pub is_pure: bool,
    is_unknown: bool
}

impl StmtType {
    pub fn from(termination: StmtTermination, is_pure: bool) -> StmtType {
        StmtType {
            termination,
            is_pure,
            is_unknown: false
        }
    }

    pub fn make_unknown() -> StmtTypeCell {
        StmtTypeCell::new(StmtType {
            termination: StmtTermination::Nonterminal,
            is_pure: false,
            is_unknown: true
        })
    }
}

impl Display for StmtType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", if self.is_pure { "Pure" } else { "Impure" })?;
        write!(f, "{}", self.termination)
    }
}

pub type TypeCell = MutCell<Type>;
pub type StmtTypeCell = MutCell<StmtType>;
