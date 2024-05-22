// use super::token::Token;

use crate::utils::loc::{Loc, Source};

struct Lexer {
    loc: Loc
}

impl Lexer {
    fn new(source: Source) -> Self {
        Lexer {
            loc: Loc {
                line: 1,
                col: 1,
                pos: 0,
                source: source.into()
            }
        }
    }
}
