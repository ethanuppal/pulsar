// Copyright (C) 2024 Ethan Uppal. This program is free software: you can
// redistribute it and/or modify it under the terms of the GNU General Public
// License as published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
use super::token::{Token, TokenType};
use pulsar_utils::{
    error::{ErrorBuilder, ErrorCode, ErrorManager, Level, Style},
    loc::{Loc, Source, Span},
    rrc::RRC
};
use std::{cell::RefCell, rc::Rc};

/// Produces tokens from an input source.
///
/// # Example
/// ```
/// fn lex(source: Rc<Source>, error_manager: RRC<ErrorManager>) {
///     let lexer = Lexer::new(source, error_manager);
///     for token in lexer {
///         println! {"{}", token};
///     }
/// }
/// ```
pub struct Lexer {
    loc: Loc,
    buffer: Vec<char>,
    error_manager: RRC<ErrorManager>
}

/// Enables exploration of the lexer buffer, e.g., with [`Lexer::advance`],
/// without side effects.
///
/// Note: this macro must only be invoked within the lexer.
macro_rules! with_unwind {
    ($self:ident in $($action:tt)*) => {
        let old_loc = $self.loc.clone();
        {
            $($action)*
        }
        $self.loc = old_loc;
    };
}

impl Lexer {
    /// Constructs a lexer for the given `source`.
    pub fn new(source: Rc<Source>, error_manager: RRC<ErrorManager>) -> Self {
        Lexer {
            loc: Loc {
                line: 1,
                col: 1,
                pos: 0,
                source: source.clone()
            },
            buffer: source.contents().chars().collect(),
            error_manager
        }
    }

    /// The current character in the buffer.
    fn current(&self) -> char {
        self.buffer[self.loc.pos as usize]
    }

    /// Whether the lexer has no remaining characters in the
    /// buffer.
    fn is_eof(&self) -> bool {
        (self.loc.pos as usize) == self.buffer.len()
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

    /// Consumes `length` characters and creates a token over those characters
    /// with type `ty`.
    fn make_token(&mut self, ty: TokenType, length: usize) -> Token {
        let loc_copy = self.loc.clone();
        self.advance_n(length);
        let pos_copy = loc_copy.pos as usize;
        let value: String =
            self.buffer[pos_copy..pos_copy + length].iter().collect();
        Token {
            ty,
            value,
            loc: loc_copy
        }
    }

    /// Requires: `current().is_numeric()`.
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

    /// Requires: `current().is_alphabetic() || current() == '_'`.
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

    /// Requires: ` current() == '@'`.
    fn make_directive_token(&mut self) -> Option<Token> {
        let mut length = 1;
        with_unwind! { self in
            self.advance();
            if self.is_eof()
                || !(self.current().is_alphanumeric() || self.current() == '_')
            {
                return None;
            }
            while !self.is_eof()
                && (self.current().is_alphanumeric() || self.current() == '_')
            {
                    self.advance();
                    length += 1;
            }
        }
        Some(self.make_token(TokenType::Directive, length))
    }
}

macro_rules! lex {
    ($self:ident in $(| $token:expr => {$token_type:expr})* | _ $finally:block) => {
        $(
            {
                let input_token_length = ($token).len();
                let loc_pos = $self.loc.pos as usize;
                if loc_pos + input_token_length <= $self.buffer.len()
                    && $self.buffer[loc_pos..loc_pos + input_token_length].iter().copied().eq($token.chars()) {
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
        if self.is_eof() || self.error_manager.borrow().has_errors() {
            return None;
        }

        self.skip();

        lex! { self in
            | "+" => { TokenType::Plus }
            | "->" => { TokenType::Arrow }
            | "-" => { TokenType::Minus }
            | "*" => { TokenType::Times }
            | "(" => { TokenType::LeftPar }
            | ")" => { TokenType::RightPar }
            | "{" => { TokenType::LeftBrace }
            | "}" => { TokenType::RightBrace }
            | "[" => { TokenType::LeftBracket }
            | "]" => { TokenType::RightBracket }
            | "<" => { TokenType::LeftAngle }
            | ">" => { TokenType::RightAngle }
            | "=" => { TokenType::Assign }
            | ":" => { TokenType::Colon }
            | "..." => { TokenType::Dots }
            | "." => { TokenType::Dot }
            | "," => { TokenType::Comma }
            | "\n" => { TokenType::Newline }
            | "func" => { TokenType::Func }
            | "let" => { TokenType::Let }
            | "return" => { TokenType::Return }
            | "pure" => { TokenType::Pure }
            | "map" => { TokenType::HardwareMap }
            | _ {
                if self.current().is_numeric() {
                    Some(self.make_number_token())
                } else if self.current().is_alphabetic() || self.current() == '_' {
                    Some(self.make_identifier_token())
                } else if self.current() == '@' {
                    self.make_directive_token()
                } else {
                    let error = ErrorBuilder::new()
                        .of_style(Style::Primary)
                        .at_level(Level::Error)
                        .with_code(ErrorCode::UnrecognizedCharacter)
                        .span(&Span::unit(self.loc.clone()))
                        .message("Encountered unrecognized character".into())
                        .build();
                    self.error_manager.borrow_mut().record(error);
                    None
                }
            }
        }
    }
}
