#[cfg(test)]
mod tests {
    use insta::assert_snapshot;
    use pulsar_frontend::lexer::Lexer;
    use pulsar_utils::{error::ErrorManager, loc::Source};
    use std::{cell::RefCell, fs, rc::Rc};

    fn read(filename: &str) -> Rc<Source> {
        Source::file(
            filename.into(),
            fs::read_to_string(filename)
                .expect(format!("Could not read file: {}", filename).as_str())
        )
    }

    fn lexer_output(
        filename: &str, error_manager: RRC<ErrorManager>
    ) -> String {
        let source = read(filename);
        let lexer = Lexer::new(source, error_manager);
        let mut output = String::new();
        for token in lexer {
            output.push_str(&format!("{:?}\n", token));
        }
        output
    }

    use paste::paste;

    macro_rules! generate_test {
        ($num:expr) => {
            paste! {
                #[test]
                fn [<test_parser_ $num>]() {
                    let error_manager = ErrorManager::with_max_count(10);
                    assert_snapshot!(lexer_output(
                        &format!("tests/data/lexer{}.plsr", $num),
                        error_manager
                    ));
                }
            }
        };
    }

    generate_test!(1);
    generate_test!(2);
    generate_test!(3);
}
