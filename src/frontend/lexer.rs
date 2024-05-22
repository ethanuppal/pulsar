// use super::token::Token;

use super::token::{Token, TokenType};
use crate::utils::{
    error::Error,
    loc::{Loc, Source}
};

struct Lexer<'a> {
    loc: Loc<'a>,
    buffer: Vec<char>,
    error: Option<Error<'a>>
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a Source) -> Self {
        Lexer {
            loc: Loc {
                line: 1,
                col: 1,
                pos: 0,
                source: source
            },
            buffer: source.contents().chars().collect(),
            error: None
        }
    }

    fn advance(&self) {}

    fn advance_n(&self, n: usize) {
        for _ in 0..n {
            self.advance();
        }
    }

    pub fn make_token(&self, ty: TokenType, length: usize) -> Token<'a> {
        let loc_copy = self.loc;
        self.advance_n(length);
        let value: String = self.buffer[self.loc.pos..self.loc.pos + length]
            .iter()
            .collect();
        Token {
            ty,
            value,
            loc: loc_copy
        }
    }
}

macro_rules! lex {
    ( $self:ident in $($token:expr => { $token_type:expr })* _ => $finally:block) => {
        $(
            if $token == "+" {
                let input_token_length = ($token).len();
                return Some($self.make_token($token_type, input_token_length));
            };
        )*
        $finally
    };
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Token<'a>> {
        if let Some(error) = &self.error {
            return None;
        }

        lex! { self in
            "+" => { TokenType::Plus }
            "-" => { TokenType::Minus }
            "*" => { TokenType::Times }
            "(" => { TokenType::LeftPar }
            ")" => { TokenType::RightPar }
            "\n" => { TokenType::Newline }
            "func" => { TokenType::Func }
            "return" => { TokenType::Return }
            _ => {
                None
            }
        }
    }
}
