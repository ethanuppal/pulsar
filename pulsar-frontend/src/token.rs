// Copyright (C) 2024 Ethan Uppal. All rights reserved.
use core::{fmt, fmt::Debug};
use pulsar_utils::loc::{Loc, RegionProvider};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    Identifier,
    Integer,
    Float,
    Bool,
    Char,
    String,
    Func,
    Let,
    Return,
    HardwareMap,
    Plus,
    Minus,
    Times,
    Assign,
    LeftPar,
    RightPar,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    LeftAngle,
    RightAngle,
    Dot,
    Dots,
    Colon,
    Comma,
    Arrow,
    Directive,
    Pure,
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
                Self::Let => "let",
                Self::Return => "return",
                Self::HardwareMap => "map",
                Self::Plus => "plus",
                Self::Minus => "minus",
                Self::Times => "times",
                Self::Assign => "assign",
                Self::LeftPar => "left-par",
                Self::RightPar => "right-par",
                Self::LeftBrace => "left-brace",
                Self::RightBrace => "right-brace",
                Self::LeftBracket => "left-bracket",
                Self::RightBracket => "right-bracket",
                Self::LeftAngle => "left-angle",
                Self::RightAngle => "right-angle",
                Self::Colon => "colon",
                Self::Dot => "dot",
                Self::Dots => "dots",
                Self::Comma => "comma",
                Self::Arrow => "arrow",
                Self::Directive => "directive",
                Self::Pure => "pure",
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
                Self::LeftBracket => "[",
                Self::RightBracket => "]",
                Self::LeftAngle => "<",
                Self::RightAngle => ">",
                Self::Colon => ":",
                Self::Assign => "=",
                Self::Dot => ".",
                Self::Dots => "...",
                Self::Comma => ",",
                Self::Arrow => "->",
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

impl RegionProvider for Token {
    fn start(&self) -> Loc {
        self.loc.clone()
    }

    fn end(&self) -> Loc {
        let mut end = self.loc.clone();
        end.pos += self.length() as isize;
        end.col += self.length() as isize;
        end
    }
}
