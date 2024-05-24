#[cfg(test)]
mod tests {
    use insta::assert_snapshot;
    use pulsar::{
        frontend::{infer::TypeInferer, lexer::Lexer, parser::Parser},
        utils::{error::ErrorManager, loc::Source}
    };
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
        let mut type_inferer = TypeInferer::new(error_manager.clone());
        let annotated_ast_opt = type_inferer.infer(program_ast);

        let mut output = String::new();
        if let Some(annotated_ast) = annotated_ast_opt {
            for node in annotated_ast {
                output.push_str(node.to_string().as_str());
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
            "tests/data/infer1.pl",
            error_manager.clone()
        ));

        assert_snapshot!(typeinferer_output(
            "tests/data/infer2.pl",
            error_manager.clone()
        ));

        assert_snapshot!(typeinferer_output(
            "tests/data/infer3.pl",
            error_manager.clone()
        ));

        assert_snapshot!(typeinferer_output(
            "tests/data/infer4.pl",
            error_manager.clone()
        ));
    }
}
