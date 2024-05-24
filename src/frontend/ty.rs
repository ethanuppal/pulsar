use crate::utils::{id::Id, mutcell::MutCell};
use lazy_static::lazy_static;
use std::{cell::RefCell, fmt::Display, hash::Hash, rc::Rc};

lazy_static! {
    pub static ref INT64_TYPE_CELL: TypeCell = TypeCell::new(Type::Int64);
}

pub const ARRAY_TYPE_UNKNOWN_SIZE: isize = -1;

#[derive(Clone, Hash, PartialEq, Eq)]
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
                "({}) -> {}{}",
                args.iter()
                    .map(|ty| ty.to_string())
                    .collect::<Vec<_>>()
                    .join(", "),
                ret,
                if *is_pure { " pure" } else { "" }
            )
        }
    }
}

#[derive(Clone)]
pub enum StmtType {
    Unknown,
    Terminal,
    Nonterminal
}

impl StmtType {
    pub fn make_unknown() -> Rc<RefCell<StmtType>> {
        Rc::new(RefCell::new(Self::Unknown))
    }
}

impl Display for StmtType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown => "?",
            Self::Terminal => "Terminal",
            Self::Nonterminal => "Nonterminal"
        }
        .fmt(f)
    }
}

pub type TypeCell = MutCell<Type>;
