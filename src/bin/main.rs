use pulsar::{frontend::lexer::Lexer, utils::loc::Source};
use std::fs;

pub fn main() {
    let filename = "data/test.pl";
    let source = Source::file(
        filename.into(),
        fs::read_to_string(filename).expect("Could not read file")
    );
    let lexer = Lexer::new(source);
    for token in lexer {
        println!("{:?}", token);
    }
}
