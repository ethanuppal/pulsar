//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

use core::{fmt, fmt::Debug};
use pulsar_utils::span::{Loc, SpanProvider};

macro_rules! token_type_enum {
    ($($(@$doc:expr =>)? $name:ident),+) => {
        #[derive(Clone, Copy, PartialEq, Eq, Debug)]
        pub enum TokenType {
            $(
                $(#[doc = $doc])?
                $name
            ),+
        }

        impl TokenType {
            /// A kebab-case name for this token type.
            pub fn name(&self) -> String {
                match self {
                    $(
                        TokenType::$name => {
                            let mut result = String::new();
                            for (i, c) in stringify!($name).chars().enumerate() {
                                if c.is_uppercase() {
                                    if i > 0 {
                                        result.push('-');
                                    }
                                    result.push(c.to_ascii_lowercase());
                                } else {
                                    result.push(c);
                                }
                            }
                            result
                        }
                    )*
                }
            }

            /// e.g., `TokenType::from_pattern("TokenType::Newline")`.
            pub fn from_pattern<S: AsRef<str>>(pat: S) -> Option<Self> {
                match pat.as_ref() {
                    $(
                        concat!("TokenType::", stringify!($name)) => Some(TokenType::$name),
                    )*
                    _ => None
                }
            }
        }
    };
}

token_type_enum! {
    Identifier,
    @"While it is guaranteed that tokens of this type represent valid integers, they may not fit within the 64-bit signed or unsigned limit." =>
    Integer,
    Float,
    Bool,
    Char,
    String,
    Func,
    Let,
    For,
    In,
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
    DotsUntil,
    Divider,
    Colon,
    Comma,
    Arrow,
    Newline
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let default = self.name();
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
            "({}, ty = {}, loc = {})",
            if self.value == "\n" {
                "\\n"
            } else {
                self.value.as_str()
            },
            self.ty.name(),
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
