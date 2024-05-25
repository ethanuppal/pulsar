use super::operand::Operand;
use std::fmt::Display;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum BranchCondition {
    Always,
    Never,
    Conditional(Operand)
}

impl Display for BranchCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Always => write!(f, "always"),
            Self::Never => write!(f, "never"),
            Self::Conditional(condition) => write!(f, "if {} != 0", condition)
        }
    }
}
