#[cfg(test)]
mod tests {
    use insta::assert_snapshot;
    use pulsar_frontend::{lexer::Lexer, parser::Parser};
    use pulsar_utils::{error::ErrorManager, loc::Source};
    use std::{cell::RefCell, fs, rc::Rc};

    pub struct Context {}

    impl AsTypePool for Context {}
    impl AsNodePool<Expr> for Context {}
    impl AsNodePool<Stmt> for Context {}
    impl AsASTPool for Context {}
    impl AsPool<TypeConstraint, ()> for Context {}
    impl AsPool<LiquidTypeConstraint, ()> for Context {}
    impl AsInferencePool for Context {}

    fn read(filename: &str) -> Rc<Source> {
        Source::file(
            filename.into(),
            fs::read_to_string(filename)
                .expect(format!("Could not read file: {}", filename).as_str())
        )
    }

    fn parser_output(
        filename: &str, error_manager: &mut ErrorManager
    ) -> String {
        let ctx = Context::default();
        let source = read(filename);
        let lexer = Lexer::new(source, error_manager);
        let tokens: Vec<_> = lexer.into_iter().collect();
        let parser = Parser::new(tokens, ctx, error_manager);
        let mut output = String::new();
        for node in parser {
            output.push_str(&format!("{}\n", node));
        }

        let mut buffer = Vec::new();
        if error_manager.borrow().has_errors() {
            error_manager
                .borrow_mut()
                .consume_and_write(&mut buffer)
                .unwrap_or_default();
        }
        output.push_str(String::from_utf8(buffer).unwrap().as_str());
        output
    }

    use paste::paste;

    macro_rules! generate_test {
        ($num:expr) => {
            paste! {
                #[test]
                fn [<test_parser_ $num>]() {
                    let mut error_manager = ErrorManager::with_max_count(10);
                    assert_snapshot!(parser_output(
                        &format!("tests/data/parser{}.plsr", $num),
                        &mut error_manager
                    ));
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
}
