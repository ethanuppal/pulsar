use crate::utils::loc::Loc;
use core::fmt;
use std::fmt::Display;

#[derive(Clone, Debug)]
pub enum TokenType {
    Identifier,
    Integer,
    Float,
    Bool,
    Char,
    String,
    Func,
    Return,
    Plus,
    Minus,
    Times,
    LeftPar,
    RightPar,
    LeftBrace,
    RightBrace,
    Newline
}

impl Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Identifier => "identifier",
                Self::Integer => "integer",
                Self::Float => "float",
                Self::Bool => "bool",
                Self::Char => "char",
                Self::String => "string",
                Self::Func => "func",
                Self::Return => "return",
                Self::Plus => "plus",
                Self::Minus => "minus",
                Self::Times => "times",
                Self::LeftPar => "left-par",
                Self::RightPar => "right-par",
                Self::LeftBrace => "left-brace",
                Self::RightBrace => "right-brace",
                Self::Newline => "\\n"
            }
        )
    }
}

pub struct Token<'a> {
    pub ty: TokenType,
    pub value: String,
    pub loc: Loc<'a>
}

impl Display for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}, ty = {}, loc = {})",
            if self.value == "\n" {
                "\\n"
            } else {
                self.value.as_str()
            },
            self.ty,
            self.loc
        )
    }
}
