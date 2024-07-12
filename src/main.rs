// Copyright (C) 2024 Ethan Uppal. This program is free software: you can
// redistribute it and/or modify it under the terms of the GNU General Public
// License as published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
use pulsar_backend::{
    calyx::{CalyxBackend, CalyxBackendInput},
    Output, PulsarBackend
};
use pulsar_frontend::{
    lexer::Lexer, parser::Parser, type_inferer::TypeInferer
};
use pulsar_ir::generator::Generator;
use pulsar_utils::{error::ErrorManager, loc::Source};
use std::{
    cell::RefCell, env, fs, io::stdout, path::PathBuf, process::Command, rc::Rc
};

fn handle_errors(error_manager: RRC<ErrorManager>) -> Result<(), ()> {
    if error_manager.borrow().has_errors() {
        error_manager
            .borrow_mut()
            .consume_and_write(&mut stdout())
            .map_err(|_| ())?;
        return Err(());
    }
    Ok(())
}

#[allow(clippy::result_unit_err)]
pub fn main() -> Result<(), ()> {
    let mut args = env::args();
    args.next(); // ignore program path
    let filename = args.next().unwrap_or("data/test.plsr".into());
    let source = Source::file(
        filename.clone(),
        fs::read_to_string(filename).expect("Could not read file")
    );

    let error_manager = ErrorManager::with_max_count(50);

    let lexer = Lexer::new(source, error_manager.clone());
    let tokens: Vec<_> = lexer.into_iter().collect();
    handle_errors(error_manager.clone())?;

    let parser = Parser::new(tokens, error_manager.clone());
    let program_ast: Vec<_> = parser.into_iter().collect();
    handle_errors(error_manager.clone())?;

    let mut type_inferer = TypeInferer::new(error_manager.clone());
    let annotated_ast =
        type_inferer.infer(program_ast).ok_or(()).map_err(|()| {
            let _ = handle_errors(error_manager.clone());
        })?;
    handle_errors(error_manager)?;

    let generator = Generator::new(annotated_ast);
    let generated_code: Vec<_> = generator.into_iter().collect();

    let command_output = Command::new("fud")
        .args(["c", "global.root"])
        .output()
        .expect("'fud' is not installed and/or misconfigured");
    let calyx_root = String::from_utf8_lossy(&command_output.stdout)
        .trim()
        .to_string();

    let calyx_backend = CalyxBackend::new(CalyxBackendInput {
        lib_path: PathBuf::from(calyx_root)
    });
    calyx_backend
        .run(generated_code, Output::Stdout)
        .map_err(|err| {
            println!("{:?}\n", err);
        })?;

    Ok(())
}
