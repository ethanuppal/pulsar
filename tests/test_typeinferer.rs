#[cfg(test)]
mod tests {
    use insta::assert_snapshot;
    use pulsar_frontend::{
        lexer::Lexer, parser::Parser, static_analysis::StaticAnalyzer
    };
    use pulsar_utils::{error::ErrorManager, loc::Source};
    use std::{cell::RefCell, fs, rc::Rc};

    fn read(filename: &str) -> Rc<Source> {
        Source::file(
            filename.into(),
            fs::read_to_string(filename)
                .expect(format!("Could not read file: {}", filename).as_str())
        )
    }

    fn typeinferer_output(
        filename: &str, error_manager: Rc<RefCell<ErrorManager>>
    ) -> String {
        let source = read(filename);
        let lexer = Lexer::new(source, error_manager.clone());
        let tokens: Vec<_> = lexer.into_iter().collect();
        let parser = Parser::new(tokens, error_manager.clone());
        let program_ast: Vec<_> = parser.into_iter().collect();
        assert_eq!(false, error_manager.borrow().has_errors());
        let mut type_inferer = StaticAnalyzer::new(error_manager.clone());
        let annotated_ast_opt = type_inferer.infer(program_ast);

        let mut output = String::new();
        if let Some(annotated_ast) = annotated_ast_opt {
            for node in annotated_ast {
                output.push_str(&format!("{}\n", node));
            }
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
                    let error_manager = ErrorManager::with_max_count(10);
                    assert_snapshot!(typeinferer_output(
                        &format!("tests/data/infer{}.plsr", $num),
                        error_manager
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
    generate_test!(11);
    generate_test!(12);
    generate_test!(13);
    generate_test!(14);
    generate_test!(15);
    generate_test!(16);
}
