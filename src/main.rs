//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

// use pulsar_backend::{
//     calyx::{CalyxBackend, CalyxBackendInput},
//     Output, PulsarBackend
// };
use pulsar_frontend::{
    lexer::Lexer, parser::Parser, type_inferer::TypeInferer
};
use pulsar_ir::from_ast;
use pulsar_lang::{context::Context, utils::OptionCheckError};
use pulsar_utils::{
    error::ErrorManager,
    loc::Source,
    pool::{AsPool, Pool}
};
use std::env;

pub fn main() -> anyhow::Result<()> {
    let mut pool = Pool::<[i32; 7], ()>::new()?;
    pool.add([1, 2, 3, 4, 6, 7, 8]);
    panic!("utku");

    let mut args = env::args();
    args.next(); // ignore program path
    let filename = args.next().unwrap_or("data/test.plsr".into());
    let source = Source::load_file(filename.clone())?;

    let mut ctx = Context::new()?;

    let mut error_manager = ErrorManager::with_max_count(50);

    let tokens = Lexer::new(source, &mut ctx, &mut error_manager)
        .lex()
        .check_errors(&mut error_manager)?;

    let ast = Parser::new(tokens, &mut ctx, &mut error_manager)
        .parse()
        .check_errors(&mut error_manager)?;

    let ast = TypeInferer::new(ast, &mut ctx, &mut error_manager)
        .infer()
        .check_errors(&mut error_manager)?;

    let comps = from_ast::ast_to_ir(ast, &mut ctx);

    for comp in comps {
        println!("{}", comp);
    }

    // let command_output = Command::new("fud")
    //     .args(["c", "global.root"])
    //     .output()
    //     .expect("'fud' is not installed and/or misconfigured");
    // let calyx_root = String::from_utf8_lossy(&command_output.stdout)
    //     .trim()
    //     .to_string();

    // let calyx_backend = CalyxBackend::new(CalyxBackendInput {
    //     lib_path: PathBuf::from(calyx_root)
    // });
    // calyx_backend
    //     .run(generated_code, Output::Stdout)
    //     .map_err(|err| {
    //         println!("{:?}\n", err);
    //     })?;

    Ok(())
}
