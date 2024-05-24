use pulsar::{
    frontend::{infer::TypeInferer, lexer::Lexer, parser::Parser},
    utils::{error::ErrorManager, loc::Source}
};
use std::{fs, io::stdout};

pub fn main() -> Result<(), ()> {
    let filename = "data/test.pl";
    let source = Source::file(
        filename.into(),
        fs::read_to_string(filename).expect("Could not read file")
    );

    let error_manager = ErrorManager::with_max_count(50);

    let lexer = Lexer::new(source, error_manager.clone());
    let tokens: Vec<_> = lexer.into_iter().collect();
    if error_manager.borrow().has_errors() {
        error_manager
            .borrow_mut()
            .consume_and_write(&mut stdout())
            .map_err(|_| ())?;
        return Err(());
    }

    let parser = Parser::new(tokens, error_manager.clone());
    let program_ast: Vec<_> = parser.into_iter().collect();
    if error_manager.borrow().has_errors() {
        error_manager
            .borrow_mut()
            .consume_and_write(&mut stdout())
            .map_err(|_| ())?;
        return Err(());
    }

    let mut type_inferer = TypeInferer::new(error_manager);
    type_inferer.infer(program_ast);

    Ok(())
}
