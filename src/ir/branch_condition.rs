use super::operand::Operand;

pub enum BranchCondition {
    Always,
    Never,
    Conditional(Operand)
}
