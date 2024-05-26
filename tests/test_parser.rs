#[cfg(test)]
mod tests {
    use insta::assert_snapshot;
    use pulsar::{
        frontend::{lexer::Lexer, parser::Parser},
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

    fn parser_output(
        filename: &str, error_manager: Rc<RefCell<ErrorManager>>
    ) -> String {
        let source = read(filename);
        let lexer = Lexer::new(source, error_manager.clone());
        let tokens: Vec<_> = lexer.into_iter().collect();
        let parser = Parser::new(tokens, error_manager.clone());
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

    #[test]
    fn test_parser() {
        let error_manager = ErrorManager::with_max_count(5);
        assert_snapshot!(parser_output(
            "tests/data/parserfail_in1.plsr",
            error_manager.clone()
        ));

        assert_snapshot!(parser_output(
            "tests/data/parser_in1.plsr",
            error_manager.clone()
        ));

        assert_snapshot!(parser_output(
            "tests/data/parser_in2.plsr",
            error_manager.clone()
        ));

        assert_snapshot!(parser_output(
            "tests/data/parser_in3.plsr",
            error_manager.clone()
        ));

        assert_snapshot!(parser_output(
            "tests/data/parser_in4.plsr",
            error_manager.clone()
        ));

        assert_snapshot!(parser_output(
            "tests/data/parser_in5.plsr",
            error_manager.clone()
        ));

        assert_snapshot!(parser_output(
            "tests/data/parser_in6.plsr",
            error_manager.clone()
        ));

        assert_snapshot!(parser_output(
            "tests/data/parser_in7.plsr",
            error_manager.clone()
        ));
    }
}
