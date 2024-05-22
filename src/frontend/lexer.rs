// use super::token::Token;

use super::token::{Token, TokenType};
use crate::utils::{
    error::Error,
    loc::{Loc, Source}
};
use std::rc::Rc;

pub struct Lexer {
    loc: Loc,
    buffer: Vec<char>,
    error: Option<Error>
}

macro_rules! with_unwind {
    ($self:ident in $action:stmt) => {
        let old_loc = $self.loc.clone();
        {
            $action
        }
        $self.loc = old_loc;
    };
}

impl Lexer {
    /// Creates a new lexer for the given `source`.
    pub fn new(source: Rc<Source>) -> Self {
        Lexer {
            loc: Loc {
                line: 1,
                col: 1,
                pos: 0,
                source: source.clone()
            },
            buffer: source.contents().chars().collect(),
            error: None
        }
    }

    /// The current character in the buffer.
    fn current(&self) -> char {
        self.buffer[self.loc.pos]
    }

    /// Whether the lexer has no remaining characters in the
    /// buffer.
    fn is_eof(&self) -> bool {
        self.loc.pos == self.buffer.len()
    }

    /// Consumes a single character in the buffer.
    fn advance(&mut self) {
        if self.current() == '\n' {
            self.loc.col = 0;
            self.loc.line += 1;
        }
        self.loc.pos += 1;
        self.loc.col += 1;
    }

    /// Consumes `n` characters in the buffer.
    fn advance_n(&mut self, n: usize) {
        for _ in 0..n {
            self.advance();
        }
    }

    /// Skips past all non-newline whitespace.
    fn skip(&mut self) {
        while !self.is_eof()
            && self.current().is_whitespace()
            && self.current() != '\n'
        {
            self.advance();
        }
    }

    fn make_token(&mut self, ty: TokenType, length: usize) -> Token {
        let loc_copy = self.loc.clone();
        self.advance_n(length);
        let value: String = self.buffer[loc_copy.pos..loc_copy.pos + length]
            .iter()
            .collect();
        Token {
            ty,
            value,
            loc: loc_copy
        }
    }

    /// Reqiores: `current().is_numeric()`.
    fn make_number_token(&mut self) -> Token {
        let mut length = 0;
        with_unwind! { self in
            while !self.is_eof() && self.current().is_numeric() {
                self.advance();
                length += 1;
            }
        }
        self.make_token(TokenType::Integer, length)
    }

    /// Reqiores: `current().is_alphabetic() || current() == '_'`.
    fn make_identifier_token(&mut self) -> Token {
        let mut length = 0;
        with_unwind! { self in
            while !self.is_eof()
            && (self.current().is_alphanumeric() || self.current() == '_')
            {
                self.advance();
                length += 1;
            }
        }
        self.make_token(TokenType::Identifier, length)
    }
}

macro_rules! lex {
    ( $self:ident in $($token:expr => { $token_type:expr })* _ => $finally:block) => {
        $(
            {
                let input_token_length = ($token).len();
                if $self.loc.pos + input_token_length <= $self.buffer.len()
                    && $self.buffer[$self.loc.pos..$self.loc.pos + input_token_length].iter().copied().eq($token.chars()) {

                return Some($self.make_token($token_type, input_token_length));
               };
            }
        )*
        $finally
    };
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        if self.is_eof() {
            return None;
        } else if let Some(_) = &self.error {
            return None;
        }

        self.skip();

        lex! { self in
            "+" => { TokenType::Plus }
            "-" => { TokenType::Minus }
            "*" => { TokenType::Times }
            "(" => { TokenType::LeftPar }
            ")" => { TokenType::RightPar }
            "{" => { TokenType::LeftBrace }
            "}" => { TokenType::RightBrace }
            "\n" => { TokenType::Newline }
            "func" => { TokenType::Func }
            "return" => { TokenType::Return }
            _ => {
                if self.current().is_numeric() {
                    Some(self.make_number_token())
                } else if self.current().is_alphabetic() || self.current() == '_' {
                    Some(self.make_identifier_token())
                } else {
                    None
                }
            }
        }
    }
}
