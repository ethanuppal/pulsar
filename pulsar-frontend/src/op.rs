//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.
use super::token::TokenType;

/// A precedence value must be strictly nonnegative.
pub type Precedence = i32;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Associativity {
    Left,
    Right
}

pub struct InfixBinaryOp {
    pub precedence: Precedence,
    pub associativity: Associativity,
    pub name: Option<String>,
    pub required_token_ty: Option<TokenType>,
    pub is_sequential: bool
}

pub struct PrefixUnaryOp {}

pub struct PostfixBinaryOp {
    pub close_token_ty: TokenType,
    pub name: Option<String>
}

/// Parsing information for an operator. At least one member must be defined.
#[derive(Default)]
pub struct Op {
    pub infix_binary: Option<InfixBinaryOp>,
    pub prefix_unary: Option<PrefixUnaryOp>,
    pub postfix_binary: Option<PostfixBinaryOp>
}

impl Op {
    pub fn infix_binary(
        mut self, precedence: Precedence, associativity: Associativity,
        name: Option<String>, required_token_ty: Option<TokenType>,
        is_sequential: bool
    ) -> Self {
        self.infix_binary = Some(InfixBinaryOp {
            precedence,
            associativity,
            name,
            required_token_ty,
            is_sequential
        });
        self
    }

    pub fn prefix_unary(mut self) -> Self {
        self.prefix_unary = Some(PrefixUnaryOp {});
        self
    }

    pub fn postfix_binary(
        mut self, close_token_ty: TokenType, name: Option<String>
    ) -> Self {
        self.postfix_binary = Some(PostfixBinaryOp {
            close_token_ty,
            name
        });
        self
    }

    /// Constructs an operator from the given token type `ty` if one exists.
    pub fn from(ty: TokenType) -> Option<Op> {
        match ty {
            TokenType::Plus | TokenType::Minus => Some(
                Op::default()
                    .infix_binary(50, Associativity::Left, None, None, false)
                    .prefix_unary()
            ),
            TokenType::Times => Some(Op::default().infix_binary(
                100,
                Associativity::Left,
                Some("multiplication".into()),
                None,
                true
            )),
            TokenType::LeftBracket => Some(Op::default().postfix_binary(
                TokenType::RightBracket,
                Some("subscript".into())
            )),
            TokenType::Dot => Some(Op::default().infix_binary(
                150,
                Associativity::Left,
                Some("member access".into()),
                Some(TokenType::Identifier),
                false
            )),
            _ => None
        }
    }

    pub fn is_unary_prefix(&self) -> bool {
        self.prefix_unary.is_some()
    }

    pub fn is_infix_binary(&self) -> bool {
        self.infix_binary.is_some()
    }

    pub fn is_postfix_binary(&self) -> bool {
        self.postfix_binary.is_some()
    }
}
