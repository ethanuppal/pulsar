//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

use super::token::{Token, TokenType};
use pulsar_utils::{
    error::{ErrorBuilder, ErrorCode, ErrorManager, Level, Style},
    pool::{AsPool, Handle, HandleArray},
    span::{Loc, Source, Span}
};
use std::rc::Rc;

/// Produces tokens from an input source.
///
/// # Example
/// ```
/// use pulsar_frontend::{lexer::Lexer, token::Token};
/// use pulsar_utils::{
///     error::ErrorManager,
///     loc::Source,
///     pool::{AsPool, HandleArray}
/// };
/// use std::rc::Rc;
///
/// fn lex<C: AsPool<Token, ()>>(
///     source: Rc<Source>, ctx: &mut C, error_manager: &mut ErrorManager
/// ) -> Option<HandleArray<Token>> {
///     Lexer::new(source, ctx, error_manager).lex()
/// }
/// ```
pub struct Lexer<'err, 'pool, P: AsPool<Token, ()>> {
    loc: Loc,
    buffer: Vec<char>,
    pool: &'pool mut P,
    error_manager: &'err mut ErrorManager
}

/// Enables exploration of the lexer buffer, e.g., with [`Lexer::advance`],
/// without side effects.
///
/// Note: this macro must only be invoked within the lexer.
macro_rules! with_unwind {
    ($self:ident; $($action:tt)*) => {
        let old_loc = $self.loc.clone();
        {
            $($action)*
        }
        $self.loc = old_loc;
    };
}

impl<'err, 'pool, P: AsPool<Token, ()>> Lexer<'err, 'pool, P> {
    /// Constructs a lexer for the given `source`.
    pub fn new(
        source: Rc<Source>, pool: &'pool mut P,
        error_manager: &'err mut ErrorManager
    ) -> Self {
        Lexer {
            loc: Loc {
                line: 1,
                col: 1,
                pos: 0,
                source: source.clone()
            },
            buffer: source.contents().chars().collect(),
            pool,
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

    /// Whether the following characters match those provided by `iter`.
    fn next_are<I: Iterator<Item = char>>(&self, iter: I) -> bool {
        for (i, c) in iter.enumerate() {
            if (self.loc.pos as usize) + i >= self.buffer.len()
                || self.buffer[(self.loc.pos as usize) + i] != c
            {
                return false;
            }
        }
        true
    }

    /// Whether a character `offset` ahead of current, if one exists, satisfies
    /// the predicate `f`.
    fn peek_matches<F: FnOnce(char) -> bool>(
        &self, offset: usize, f: F
    ) -> bool {
        if (self.loc.pos as usize) + offset >= self.buffer.len() {
            false
        } else {
            f(self.buffer[(self.loc.pos as usize) + offset])
        }
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
    fn make_token(&mut self, ty: TokenType, length: usize) -> Handle<Token> {
        let loc_copy = self.loc.clone();
        self.advance_n(length);
        let pos_copy = loc_copy.pos as usize;
        let value: String =
            self.buffer[pos_copy..pos_copy + length].iter().collect();
        self.pool.add(Token {
            ty,
            value,
            loc: loc_copy
        })
    }

    /// Requires: `current().is_numeric()`.
    fn make_number_token(&mut self) -> Handle<Token> {
        let mut length = 0;
        with_unwind! { self;
            while !self.is_eof() && self.current().is_numeric() {
                self.advance();
                length += 1;
            }
        }
        self.make_token(TokenType::Integer, length)
    }

    /// Requires: `current().is_alphabetic() || current() == '_'`.
    fn make_identifier_token(&mut self) -> Handle<Token> {
        let mut length = 0;
        with_unwind! { self;
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

enum LexSemantics {
    Keyword
}

/// Implements an inefficient pattern checker and token emitter. For example,
/// ```
/// lex! { self;
///     | "..." => { TokenType::Dots }
///     | "." => { TokenType::Dot }
///     | @[LexSemantics::Keyword] "for" => { TokenType::For }
///     | @[LexSemantics::Keyword] "let" => { TokenType::Let }
///     | _ {
///         Some(self.make_token(TokenType::Lol, 400))
///     }
/// }
/// ```
/// first checks whether the input stream matches `"..."`, and if it does, it
/// consumes those 3 characters and returns `Some` token with type
/// `TokenType::Dots`; if `LexSemantics::Keyword` is specified, either EOF or
/// whitespace is required after the pattern; the final `_` block does not have
/// an arrow and wraps a rust expression that returns an
/// `Option<Handle<Token>>`.
macro_rules! lex {
    ($self:ident; $(| $(@[$semantics:expr])? $token:expr => {$token_type:expr})* | _ $finally:block) => {
        $(
            {
                let input_token_length = ($token).len();
                let loc_pos = $self.loc.pos as usize;
                if loc_pos + input_token_length <= $self.buffer.len()
                    && $self.next_are($token.chars())
                    $(&& match $semantics {
                        LexSemantics::Keyword => $self.peek_matches(input_token_length, |c| c.is_whitespace()),
                        #[allow(unreachable_patterns)]
                        _ => true
                    })? {
                    return Some($self.make_token($token_type, input_token_length));
                };
            }
        )*
        $finally
    };
}

impl<'err, 'pool, P: AsPool<Token, ()>> Lexer<'err, 'pool, P> {
    fn next_token(&mut self) -> Option<Handle<Token>> {
        if self.is_eof() || self.error_manager.has_errors() {
            return None;
        }

        self.skip();

        lex! { self;
            | "+" => { TokenType::Plus }
            | "->" => { TokenType::Arrow }
            | "---" => { TokenType::Divider }
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
            | "..<" => { TokenType::DotsUntil }
            | "." => { TokenType::Dot }
            | "," => { TokenType::Comma }
            | ";" => { TokenType::Semicolon }
            | "\n" => { TokenType::Newline }
            | @[LexSemantics::Keyword] "func" => { TokenType::Func }
            | @[LexSemantics::Keyword] "let" => { TokenType::Let }
            | @[LexSemantics::Keyword] "for" => { TokenType::For }
            | @[LexSemantics::Keyword] "in" => { TokenType::In }
            | _ {
                if self.current().is_numeric() {
                    Some(self.make_number_token())
                } else if self.current().is_alphabetic() || self.current() == '_' {
                    Some(self.make_identifier_token())
                } else {
                    let error = ErrorBuilder::new()
                        .of_style(Style::Primary)
                        .at_level(Level::Error)
                        .with_code(ErrorCode::UnrecognizedCharacter)
                        .span(Span::unit(self.loc.clone()))
                        .message("Encountered unrecognized character")
                        .build();
                    self.error_manager.record(error);
                    None
                }
            }
        }
    }

    pub fn lex(mut self) -> Option<HandleArray<Token>> {
        while self.next_token().is_some() {}
        if self.error_manager.has_errors() {
            None
        } else {
            Some(self.pool.as_pool_mut().as_array())
        }
    }
}
