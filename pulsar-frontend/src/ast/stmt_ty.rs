use std::fmt::Display;

use pulsar_utils::pool::Handle;

use super::ty::Type;

#[derive(Clone)]
pub enum StmtType {
    Terminal(Handle<Type>),
    Nonterminal
}

impl Display for StmtType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Terminal(ty) => write!(f, "Terminal({})", ty),
            Self::Nonterminal => write!(f, "Nonterminal")
        }
    }
}
