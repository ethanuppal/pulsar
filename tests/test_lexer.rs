//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

#[cfg(test)]
mod tests {
    use insta::assert_snapshot;
    use pulsar_frontend::lexer::Lexer;
    use pulsar_lang::context::Context;
    use pulsar_utils::{error::ErrorManager, span::Source};
    use std::fmt::Write;

    fn lexer_output(filename: &str) -> String {
        let mut ctx = Context::new().unwrap();
        let source = Source::load_file(filename)
            .unwrap_or_else(|_| panic!("Could not read file: {}", filename));
        let mut error_manager = ErrorManager::with_max_count(10);
        let tokens = Lexer::new(source, &mut ctx, &mut error_manager)
            .lex()
            .expect("invalid input");
        let mut output = String::new();
        for token in tokens {
            writeln!(&mut output, "{:?}", token).unwrap();
        }
        output
    }

    use paste::paste;

    macro_rules! generate_test {
        ($num:expr) => {
            paste! {
                #[test]
                fn [<test_lexer_ $num>]() {
                    assert_snapshot!(lexer_output(
                        &format!("tests/data/lexer{}.plsr", $num)
                    ));
                }
            }
        };
    }

    generate_test!(1);
    generate_test!(2);
    generate_test!(3);
}
