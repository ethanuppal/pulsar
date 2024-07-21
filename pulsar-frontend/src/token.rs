//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

use core::{fmt, fmt::Debug};
use pulsar_utils::loc::{Loc, SpanProvider};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TokenType {
    #[serde(rename = "identifier")]
    Identifier,
    /// While it is guaranteed that tokens of this type represent valid
    /// integers, they may not fit within the 64-bit signed or unsigned limit.
    #[serde(rename = "integer")]
    Integer,
    #[serde(rename = "float")]
    Float,
    #[serde(rename = "bool")]
    Bool,
    #[serde(rename = "char")]
    Char,
    #[serde(rename = "string")]
    String,
    #[serde(rename = "func")]
    Func,
    #[serde(rename = "let")]
    Let,
    #[serde(rename = "for")]
    For,
    #[serde(rename = "in")]
    In,
    #[serde(rename = "plus")]
    Plus,
    #[serde(rename = "minus")]
    Minus,
    #[serde(rename = "times")]
    Times,
    #[serde(rename = "assign")]
    Assign,
    #[serde(rename = "left-par")]
    LeftPar,
    #[serde(rename = "right-par")]
    RightPar,
    #[serde(rename = "left-brace")]
    LeftBrace,
    #[serde(rename = "right-brace")]
    RightBrace,
    #[serde(rename = "left-bracket")]
    LeftBracket,
    #[serde(rename = "right-bracket")]
    RightBracket,
    #[serde(rename = "left-angle")]
    LeftAngle,
    #[serde(rename = "right-angle")]
    RightAngle,
    #[serde(rename = "dot")]
    Dot,
    #[serde(rename = "dots")]
    Dots,
    #[serde(rename = "dots-until")]
    DotsUntil,
    #[serde(rename = "divider")]
    Divider,
    #[serde(rename = "colon")]
    Colon,
    #[serde(rename = "comma")]
    Comma,
    #[serde(rename = "arrow")]
    Arrow,
    #[serde(rename = "newline")]
    Newline
}

impl Debug for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string(self).expect("I handled all cases")
        )
    }
}

impl Display for TokenType {
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
                Self::DotsUntil => "..<",
                Self::Divider => "---",
                Self::Comma => ",",
                Self::Arrow => "->",
                _ => default.as_str()
            }
        )
    }
}

/// A lexical unit of source code.
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

impl AsRef<Token> for Token {
    fn as_ref(&self) -> &Token {
        self
    }
}

impl SpanProvider for Token {
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
