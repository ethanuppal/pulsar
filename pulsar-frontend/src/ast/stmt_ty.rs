use std::fmt::Display;

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
    pub is_pure: bool
}

impl StmtType {
    pub fn from(termination: StmtTermination, is_pure: bool) -> StmtType {
        StmtType {
            termination,
            is_pure
        }
    }
}

impl Display for StmtType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", if self.is_pure { "Pure" } else { "Impure" })?;
        write!(f, "{}", self.termination)
    }
}
