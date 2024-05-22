use super::token::TokenType;

/// A precedence value must be strictly nonnegative.
pub type Precedence = i32;

pub struct Op {
    pub is_binary: bool,
    pub is_unary: bool,
    pub is_left_associative: bool,
    pub binary_precedence: Precedence,
    pub unary_precedence: Precedence
}

impl Op {
    pub fn from(ty: TokenType) -> Option<Op> {
        match ty {
            TokenType::Plus | TokenType::Minus => Some(Op {
                is_binary: true,
                is_unary: true,
                is_left_associative: true,
                binary_precedence: 50,
                unary_precedence: 50
            }),
            TokenType::Times => Some(Op {
                is_binary: true,
                is_unary: false,
                is_left_associative: true,
                binary_precedence: 100,
                unary_precedence: 0
            }),
            _ => None
        }
    }

    pub fn is_operator(ty: TokenType) -> bool {
        Op::from(ty).is_some()
    }
}
