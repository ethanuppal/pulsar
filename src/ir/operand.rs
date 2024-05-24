use super::variable::Variable;

pub enum Operand {
    Constant(i64),
    Variable(Variable)
}
