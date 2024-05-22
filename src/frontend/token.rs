use crate::utils::loc::Loc;
use core::fmt;
use core::fmt::Debug;

#[derive(Clone, Copy, PartialEq, Eq)]
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

impl Debug for TokenType {
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
                Self::Newline => "newline"
            }
        )
    }
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let default = format!("{:?}", self);
        write!(
            f,
            "{}",
            match self {
                Self::Plus => "+",
                Self::Minus => "-",
                Self::Times => "*",
                Self::LeftPar => "(",
                Self::RightPar => ")",
                Self::LeftBrace => "{",
                Self::RightBrace => "}",
                _ => default.as_str()
            }
        )
    }
}

#[derive(Clone)]
pub struct Token {
    pub ty: TokenType,
    pub value: String,
    pub loc: Loc
}

impl Token {
    /// ```
    /// let token = Token {
    ///     ty: TokenType::Identifier,
    ///     value: "main".to_string(),
    ///     loc: Loc::default()
    /// };
    /// assert_eq(token.value.len(), token.length());
    /// ```
    pub fn length(&self) -> usize {
        self.value.len()
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}, ty = {:?}, loc = {})",
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

pub type Name = Token;
