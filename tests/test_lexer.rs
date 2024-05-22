#[cfg(test)]
mod tests {
    use insta::assert_snapshot;
    use pulsar::utils::error::ErrorManager;
    use pulsar::{frontend::lexer::Lexer, utils::loc::Source};
    use std::cell::RefCell;
    use std::fs;
    use std::rc::Rc;

    fn read(filename: &str) -> Rc<Source> {
        Source::file(
            filename.into(),
            fs::read_to_string(filename)
                .expect(format!("Could not read file: {}", filename).as_str())
        )
    }

    fn lexer_output(
        filename: &str, error_manager: Rc<RefCell<ErrorManager>>
    ) -> String {
        let source = read(filename);
        let lexer = Lexer::new(source, error_manager);
        let mut output = String::new();
        for token in lexer {
            output.push_str(&format!("{:?}\n", token));
        }
        output
    }

    #[test]
    fn test_lexer() {
        let error_manager = ErrorManager::with_max_count(5);
        assert_snapshot!(lexer_output(
            "tests/data/lexer_in1.pl",
            error_manager.clone()
        ));
        assert_snapshot!(lexer_output(
            "tests/data/lexer_in2.pl",
            error_manager.clone()
        ));
        assert_snapshot!(lexer_output(
            "tests/data/lexer_in3.pl",
            error_manager.clone()
        ));
    }
}
