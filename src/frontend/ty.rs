use crate::utils::id::Id;
use std::{
    cell::{Ref, RefCell, RefMut},
    fmt::Display,
    hash::{Hash, Hasher},
    rc::Rc
};

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

    pub fn refcell_int64() -> TypeCell {
        TypeCell::new(Type::Int64)
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

#[derive(Clone)]
pub struct TypeCell {
    pointer: Rc<RefCell<Type>>
}

impl TypeCell {
    pub fn new(ty: Type) -> Self {
        Self {
            pointer: Rc::new(RefCell::new(ty))
        }
    }

    pub fn get(&self) -> Type {
        self.pointer.borrow().clone()
    }

    pub fn as_ref(&self) -> Ref<Type> {
        self.pointer.borrow()
    }

    pub fn as_mut(&self) -> RefMut<Type> {
        self.pointer.borrow_mut()
    }

    pub fn raw(&self) -> Rc<RefCell<Type>> {
        self.pointer.clone()
    }
}

impl PartialEq for TypeCell {
    fn eq(&self, other: &Self) -> bool {
        self.pointer.borrow().eq(&other.pointer.borrow())
    }
}
impl Eq for TypeCell {}
impl Hash for TypeCell {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.pointer.borrow().hash(state)
    }
}
impl Display for TypeCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.get().fmt(f)
    }
}
