//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

#[cfg(test)]
mod tests {
    use insta::assert_snapshot;
    use pulsar_frontend::{
        ast::{expr::Expr, node::AsNodePool},
        lexer::Lexer,
        parser::Parser,
        type_inferer::TypeInferer
    };
    use pulsar_lang::{context::Context, utils::OptionCheckError};
    use pulsar_utils::{
        error::ErrorManager,
        pool::{AsPool, HandleArray},
        span::{Source, SpanProvider}
    };
    use std::fmt::Write;

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

        let ast = TypeInferer::new(ast, &mut ctx, &mut error_manager).infer();

        let mut output = String::new();

        if let Some(ast) = ast {
            for decl in ast {
                writeln!(&mut output, "{}", decl).unwrap();
            }
        }

        if !error_manager.has_errors() {
            let exprs: HandleArray<Expr> = ctx.as_pool_mut().as_array();
            for expr in exprs {
                if ctx.get_ty(expr).is_invalid() {
                    panic!(
                        "[{}] {} did not have type resolved",
                        expr.span(),
                        expr
                    );
                }
                writeln!(
                    &mut output,
                    "{} {}: {}",
                    expr.span(),
                    expr,
                    ctx.get_ty(expr)
                )
                .unwrap();
            }
        }

        let mut buffer = Vec::new();
        error_manager.consume_and_write(&mut buffer)?;
        output.push_str(&String::from_utf8_lossy(&buffer));

        Ok(output)
    }

    use paste::paste;

    macro_rules! generate_test {
        ($num:expr) => {
            paste! {
                #[test]
                fn [<test_type_inferer_ $num>]() {
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
}
