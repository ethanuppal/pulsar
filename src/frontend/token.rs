use crate::utils::loc::Loc;
use core::fmt;
use std::fmt::Display;

#[derive(Clone, Debug)]
pub enum Literal {
    Integer,
    Float,
    Bool,
    Char,
    String
}

impl Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Integer => "integer",
                Self::Float => "float",
                Self::Bool => "bool",
                Self::Char => "char",
                Self::String => "string"
            }
        )
    }
}

#[derive(Clone, Debug)]
pub enum Keyword {
    Func,
    Return
}

impl Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Func => "func",
                Self::Return => "return"
            }
        )
    }
}

#[derive(Clone, Debug)]
pub enum Symbol {
    Plus,
    Minus,
    Times,
    LeftPar,
    RightPar
}

impl Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Plus => "plus",
                Self::Minus => "minus",
                Self::Times => "times",
                Self::LeftPar => "left-par",
                Self::RightPar => "right-par"
            }
        )
    }
}

#[derive(Clone, Debug)]
pub enum Type {
    Identifier,
    Literal(Literal),
    Keyword(Keyword),
    Symbol(Symbol),
    Newline
}

impl Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Identifier => write!(f, "identifier"),
            Self::Literal(literal) => write!(f, "{}", literal),
            Self::Keyword(keyword) => write!(f, "{}", keyword),
            Self::Symbol(symbol) => write!(f, "{}", symbol),
            Self::Newline => write!(f, "\\n")
        }
    }
}

/// `Token(ty, value, loc)` is a token of type `ty` at location `loc` with
/// contents `value`.
///
/// A token is formatted as `"(value, ty = {ty}, loc = {loc})"`, where `{ty}`
/// and `{loc}` represent the formatted substitutions of the `ty` and `loc`
/// fields of the token.
pub struct Token {
    pub ty: Type,
    pub value: String,
    pub loc: Loc
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, ty = {}, loc = {})", self.value, self.ty, self.loc)
    }
}
