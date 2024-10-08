//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

// this test was mainly written so I could learn the syntax for proptest

#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    use pulsar_frontend::token::{Token, TokenType};
    use pulsar_utils::span::{Loc, Source};
    use std::rc::Rc;

    fn arb_token_type() -> impl Strategy<Value = TokenType> {
        prop_oneof![
            Just(TokenType::Identifier),
            Just(TokenType::Integer),
            Just(TokenType::Float),
            Just(TokenType::Bool),
            Just(TokenType::Char),
            Just(TokenType::String),
            Just(TokenType::Func),
            Just(TokenType::Let),
            Just(TokenType::Plus),
            Just(TokenType::Minus),
            Just(TokenType::Times),
            Just(TokenType::Assign),
            Just(TokenType::LeftPar),
            Just(TokenType::RightPar),
            Just(TokenType::Newline),
        ]
    }

    fn arb_source() -> impl Strategy<Value = Rc<Source>> {
        prop_oneof![
            (any::<String>(), any::<String>())
                .prop_map(|(name, contents)| { Source::file(name, contents) }),
            Just(Rc::new(Source::Unknown)),
        ]
    }

    fn arb_loc() -> impl Strategy<Value = Loc> {
        (any::<usize>(), any::<usize>(), any::<usize>(), arb_source()).prop_map(
            |(line, col, pos, source)| Loc {
                line: line as isize,
                col: col as isize,
                pos: pos as isize,
                source
            }
        )
    }

    proptest! {
        #[test]
        fn loc_formats_correctly(
            loc in arb_loc()
        ) {
            assert_eq!(
                format!("{}:{}:{}", loc.source, loc.line, loc.col),
                format!("{}", loc)
            );
        }
    }

    proptest! {
        #[test]
        fn token_formats_correctly(
            ty in arb_token_type(),
            value in any::<String>(),
            loc in arb_loc(),
        ) {
            assert_eq!(
                format!("({}, ty = {}, loc = {})", value, ty.name(), loc),
                format!("{:?}",  Token { ty, value, loc })
            );
        }
    }
}
