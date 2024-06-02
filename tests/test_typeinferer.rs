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

    #[test]
    fn test_typeinferer() {
        let error_manager = ErrorManager::with_max_count(5);
        assert_snapshot!(typeinferer_output(
            "tests/data/infer1.plsr",
            error_manager.clone()
        ));

        assert_snapshot!(typeinferer_output(
            "tests/data/infer2.plsr",
            error_manager.clone()
        ));

        assert_snapshot!(typeinferer_output(
            "tests/data/infer3.plsr",
            error_manager.clone()
        ));

        assert_snapshot!(typeinferer_output(
            "tests/data/infer4.plsr",
            error_manager.clone()
        ));

        assert_snapshot!(typeinferer_output(
            "tests/data/infer5.plsr",
            error_manager.clone()
        ));

        assert_snapshot!(typeinferer_output(
            "tests/data/infer6.plsr",
            error_manager.clone()
        ));

        assert_snapshot!(typeinferer_output(
            "tests/data/infer7.plsr",
            error_manager.clone()
        ));

        assert_snapshot!(typeinferer_output(
            "tests/data/infer8.plsr",
            error_manager.clone()
        ));

        assert_snapshot!(typeinferer_output(
            "tests/data/infer9.plsr",
            error_manager.clone()
        ));

        assert_snapshot!(typeinferer_output(
            "tests/data/infer10.plsr",
            error_manager.clone()
        ));

        assert_snapshot!(typeinferer_output(
            "tests/data/infer11.plsr",
            error_manager.clone()
        ));

        assert_snapshot!(typeinferer_output(
            "tests/data/infer12.plsr",
            error_manager.clone()
        ));
    }

    #[test]
    fn test_infer13() {
        let error_manager = ErrorManager::with_max_count(5);
        assert_snapshot!(typeinferer_output(
            "tests/data/infer13.plsr",
            error_manager
        ));
    }

    #[test]
    fn test_infer14() {
        let error_manager = ErrorManager::with_max_count(5);
        assert_snapshot!(typeinferer_output(
            "tests/data/infer14.plsr",
            error_manager
        ));
    }

    #[test]
    fn test_infer15() {
        let error_manager = ErrorManager::with_max_count(5);
        assert_snapshot!(typeinferer_output(
            "tests/data/infer15.plsr",
            error_manager
        ));
    }

    #[test]
    fn test_infer16() {
        let error_manager = ErrorManager::with_max_count(5);
        assert_snapshot!(typeinferer_output(
            "tests/data/infer16.plsr",
            error_manager
        ));
    }
}
