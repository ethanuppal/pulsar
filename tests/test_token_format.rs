extern crate pulsar;

#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    use pulsar::frontend::token::{Keyword, Literal, Symbol, Token, Type};
    use pulsar::utils::loc::{Loc, Source};
    use std::rc::Rc;

    fn arb_literal() -> impl Strategy<Value = Literal> {
        prop_oneof![
            Just(Literal::Integer),
            Just(Literal::Float),
            Just(Literal::Bool),
            Just(Literal::Char),
            Just(Literal::String),
        ]
    }

    fn arb_keyword() -> impl Strategy<Value = Keyword> {
        prop_oneof![Just(Keyword::Func), Just(Keyword::Return)]
    }

    fn arb_symbol() -> impl Strategy<Value = Symbol> {
        prop_oneof![
            Just(Symbol::Plus),
            Just(Symbol::Minus),
            Just(Symbol::Times),
            Just(Symbol::LeftPar),
            Just(Symbol::RightPar),
        ]
    }

    fn arb_type() -> impl Strategy<Value = Type> {
        prop_oneof![
            Just(Type::Identifier),
            arb_literal().prop_map(Type::Literal),
            arb_keyword().prop_map(Type::Keyword),
            arb_symbol().prop_map(Type::Symbol),
            Just(Type::Newline),
        ]
    }

    fn arb_source() -> impl Strategy<Value = Rc<Source>> {
        any::<(String, String)>().prop_map(|(name, contents)| {
            Rc::new(Source::File { name, contents })
        })
    }

    fn arb_loc() -> impl Strategy<Value = Loc> {
        (
            any::<usize>(), // line
            any::<usize>(), // col
            any::<usize>(), // pos
            arb_source()    // source
        )
            .prop_map(|(line, col, pos, source)| Loc {
                line,
                col,
                pos,
                source
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
            ty in arb_type(),
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
