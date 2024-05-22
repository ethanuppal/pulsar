#[cfg(test)]
mod tests {
    use insta::assert_snapshot;
    use pulsar::{frontend::lexer::Lexer, utils::loc::Source};
    use std::fs;
    use std::rc::Rc;

    fn read(filename: &str) -> Rc<Source> {
        Source::file(
            filename.into(),
            fs::read_to_string(filename)
                .expect(format!("Could not read file: {}", filename).as_str())
        )
    }

    fn lexer_output(filename: &str) -> String {
        let source = read(filename);
        let lexer = Lexer::new(source);
        let mut output = String::new();
        for token in lexer {
            output.push_str(&format!("{:?}\n", token));
        }
        output
    }

    #[test]
    fn test_lexer() {
        assert_snapshot!(lexer_output("tests/data/in1.pl"), @r###"
        (func, ty = func, loc = tests/data/in1.pl:1:1)
        (main, ty = identifier, loc = tests/data/in1.pl:1:6)
        ((, ty = left-par, loc = tests/data/in1.pl:1:10)
        (), ty = right-par, loc = tests/data/in1.pl:1:11)
        ({, ty = left-brace, loc = tests/data/in1.pl:1:13)
        (\n, ty = \n, loc = tests/data/in1.pl:1:14)
        (this, ty = identifier, loc = tests/data/in1.pl:2:5)
        (is, ty = identifier, loc = tests/data/in1.pl:2:10)
        (a, ty = identifier, loc = tests/data/in1.pl:2:13)
        (test, ty = identifier, loc = tests/data/in1.pl:2:15)
        (\n, ty = \n, loc = tests/data/in1.pl:2:19)
        (}, ty = right-brace, loc = tests/data/in1.pl:3:1)
        (\n, ty = \n, loc = tests/data/in1.pl:3:2)
        "###);
        assert_snapshot!(lexer_output("tests/data/in2.pl"), @r###"
        (1, ty = integer, loc = tests/data/in2.pl:1:1)
        (2, ty = integer, loc = tests/data/in2.pl:1:3)
        (3, ty = integer, loc = tests/data/in2.pl:1:5)
        (4, ty = integer, loc = tests/data/in2.pl:1:7)
        (5, ty = integer, loc = tests/data/in2.pl:1:9)
        (\n, ty = \n, loc = tests/data/in2.pl:1:10)
        (id, ty = identifier, loc = tests/data/in2.pl:2:1)
        (\n, ty = \n, loc = tests/data/in2.pl:2:3)
        (id1, ty = identifier, loc = tests/data/in2.pl:3:1)
        (\n, ty = \n, loc = tests/data/in2.pl:3:4)
        (_id1, ty = identifier, loc = tests/data/in2.pl:4:1)
        (\n, ty = \n, loc = tests/data/in2.pl:4:5)
        (1, ty = integer, loc = tests/data/in2.pl:5:1)
        (id, ty = identifier, loc = tests/data/in2.pl:5:2)
        (\n, ty = \n, loc = tests/data/in2.pl:5:4)
        "###);
        assert_snapshot!(lexer_output("tests/data/in3.pl"), @r###"
        (func, ty = func, loc = tests/data/in3.pl:1:1)
        (test, ty = identifier, loc = tests/data/in3.pl:1:6)
        ((, ty = left-par, loc = tests/data/in3.pl:1:10)
        (), ty = right-par, loc = tests/data/in3.pl:1:11)
        ({, ty = left-brace, loc = tests/data/in3.pl:1:13)
        (\n, ty = \n, loc = tests/data/in3.pl:1:14)
        (print, ty = identifier, loc = tests/data/in3.pl:2:5)
        ((, ty = left-par, loc = tests/data/in3.pl:2:10)
        (1, ty = integer, loc = tests/data/in3.pl:2:11)
        (+, ty = plus, loc = tests/data/in3.pl:2:13)
        (2, ty = integer, loc = tests/data/in3.pl:2:15)
        (*, ty = times, loc = tests/data/in3.pl:2:17)
        (3, ty = integer, loc = tests/data/in3.pl:2:19)
        (), ty = right-par, loc = tests/data/in3.pl:2:20)
        (\n, ty = \n, loc = tests/data/in3.pl:2:21)
        (}, ty = right-brace, loc = tests/data/in3.pl:3:1)
        (\n, ty = \n, loc = tests/data/in3.pl:3:2)
        "###);
    }
}
