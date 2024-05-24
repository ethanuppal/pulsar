use crate::utils::id::Id;
use std::{cell::RefCell, fmt::Display, rc::Rc};

pub const ARRAY_TYPE_UNKNOWN_SIZE: isize = -1;

#[derive(Clone, Hash, PartialEq, Eq)]
pub enum Type {
    Unknown,
    Unit,
    Var(Id),
    Name(String),
    Int64,

    /// A negative size indicates that the size is not yet known.
    Array(Box<Type>, isize),

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

    pub fn make_unknown() -> Rc<RefCell<Type>> {
        Rc::new(RefCell::new(Self::Unknown))
    }

    pub fn refcell_int64() -> Rc<RefCell<Type>> {
        Rc::new(RefCell::new(Type::Int64))
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
            Self::Array(ty, size) => write!(
                f,
                "{}[{}]",
                ty,
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
