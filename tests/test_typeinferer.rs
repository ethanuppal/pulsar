//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

#[cfg(test)]
mod tests {
    use insta::assert_snapshot;
    use pulsar_frontend::{lexer::Lexer, parser::Parser};
    use pulsar_lang::{context::Context, utils::OptionCheckError};
    use pulsar_utils::{error::ErrorManager, loc::Source};

    fn typeinferer_output(filename: &str) -> anyhow::Result<String> {
        let mut ctx = Context::new().unwrap();
        let source = Source::load_file(filename)?;
        let mut error_manager = ErrorManager::with_max_count(10);
        let tokens = Lexer::new(source, &mut ctx, &mut error_manager)
            .lex()
            .check_errors(&mut error_manager)?;

        let ast = Parser::new(tokens, &mut ctx, &mut error_manager)
            .parse()
            .check_errors(&mut error_manager)?;

        let mut output = String::new();

        for decl in ast {
            output.push_str(&format!("{}\n", decl));
        }

        let mut buffer = Vec::new();
        if error_manager.has_errors() {
            error_manager.consume_and_write(&mut buffer)?
        }
        output.push_str(String::from_utf8(buffer).unwrap().as_str());

        Ok(output)
    }

    use paste::paste;

    macro_rules! generate_test {
        ($num:expr) => {
            paste! {
                #[test]
                fn [<test_parser_ $num>]() {
                    assert_snapshot!(typeinferer_output(
                        &format!("tests/data/infer{}.plsr", $num)
                    ).expect("failed to parse/infer input"));
                }
            }
        };
    }

    generate_test!(1);
    generate_test!(2);
    generate_test!(3);
    generate_test!(4);
    generate_test!(5);
    generate_test!(6);
    generate_test!(7);
    generate_test!(8);
    generate_test!(9);
    generate_test!(10);
    generate_test!(11);
    generate_test!(12);
    generate_test!(13);
    generate_test!(14);
    generate_test!(15);
    generate_test!(16);
}
