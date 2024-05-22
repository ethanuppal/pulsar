extern crate pulsar;

#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    use pulsar::frontend::token::{Token, TokenType};
    use pulsar::utils::loc::{Loc, Source};

    fn arb_token_type() -> impl Strategy<Value = TokenType> {
        prop_oneof![
            Just(TokenType::Identifier),
            Just(TokenType::Integer),
            Just(TokenType::Float),
            Just(TokenType::Bool),
            Just(TokenType::Char),
            Just(TokenType::String),
            Just(TokenType::Func),
            Just(TokenType::Return),
            Just(TokenType::Plus),
            Just(TokenType::Minus),
            Just(TokenType::Times),
            Just(TokenType::LeftPar),
            Just(TokenType::RightPar),
            Just(TokenType::Newline),
        ]
    }

    fn arb_loc() -> impl Strategy<Value = Loc<'static>> {
        (
            any::<usize>(), // line
            any::<usize>(), // col
            any::<usize>()  // pos
        )
            .prop_map(|(line, col, pos)| Loc {
                line,
                col,
                pos,
                source: &Source::Unknown
            })
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
                format!("({}, ty = {}, loc = {})", value, ty, loc),
                format!("{}",  Token { ty, value, loc })
            );
        }
    }
}
