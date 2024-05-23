use std::fmt::Display;

pub enum Type {
    Var(i64),
    Name(String),
    Int64,
    Array(Box<Type>, usize)
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Var(var) => write!(f, "'{}", var),
            Self::Name(name) => write!(f, "{}", name),
            Self::Int64 => write!(f, "Int64"),
            Self::Array(ty, size) => write!(f, "{}[{}]", ty, size)
        }
    }
}

pub enum StmtType {
    Terminal,
    Nonterminal
}

impl Display for StmtType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Terminal => "Terminal",
            Self::Nonterminal => "Nonterminal"
        }
        .fmt(f)
    }
}
