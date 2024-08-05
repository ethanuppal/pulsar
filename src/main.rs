//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

use pulsar_backend::{
    target::{calyx::CalyxTarget, print::PrintTarget, OutputFile},
    transform::agen::AddressGeneratorTransform,
    BackendBuilder
};
// use pulsar_backend::{
//     calyx::{CalyxBackend, CalyxBackendInput},
//     Output, PulsarBackend
// };
use pulsar_frontend::{
    lexer::Lexer, parser::Parser, type_inferer::TypeInferer
};
use pulsar_ir::{from_ast, pass::PassRunner};
use pulsar_lang::{context::Context, utils::OptionCheckError};
use pulsar_utils::{
    error::{ErrorCode, ErrorManager},
    id::Gen,
    span::Source
};
use std::env;

pub fn main() -> anyhow::Result<()> {
    env_logger::builder()
        .filter(None, log::LevelFilter::Info)
        .parse_env("LOG")
        .format_timestamp(None)
        .init();

    let mut args = env::args();
    args.next(); // ignore program path
    let first_arg = args.next();
    if first_arg == Some("--explain".into()) {
        let error_code = args
            .next()
            .expect("missing code after --explain")
            .parse::<i32>()
            .expect("invalid error code");
        let error_code =
            ErrorCode::from(error_code).expect("invalid error code");
        println!("Code: {}", error_code);
        println!("Description: {}", error_code.description());
        return Ok(());
    }

    let filename = first_arg.unwrap_or("data/test.plsr".into());
    let source = Source::load_file(filename.clone())?;

    let mut ctx = Context::new()?;

    let mut error_manager = ErrorManager::with_max_count(50);

    log::info!("Parsing...");

    let tokens = Lexer::new(source, &mut ctx, &mut error_manager)
        .lex()
        .check_errors(&mut error_manager)?;

    let ast = Parser::new(tokens, &mut ctx, &mut error_manager)
        .parse()
        .check_errors(&mut error_manager)?;

    log::info!("Inferring types...");

    let ast = TypeInferer::new(ast, &mut ctx, &mut error_manager)
        .infer()
        .check_errors(&mut error_manager)?;

    // {
    //     use pulsar_frontend::ast::{expr::Expr, node::AsNodePool};
    //     use pulsar_utils::{
    //         pool::{AsPool, HandleArray},
    //         span::SpanProvider
    //     };

    //     let exprs: HandleArray<Expr> = ctx.as_pool_mut().as_array();
    //     for expr in exprs {
    //         if ctx.get_ty(expr).is_invalid() {
    //             panic!("[{}] {} did not have type resolved", expr.span(),
    // expr);         }
    //         println!("{} {}: {}", expr.span(), expr, ctx.get_ty(expr));
    //     }
    // }

    // for decl in ast {
    //     println!("{}", decl);
    // }

    log::info!("Optimizing...");

    let mut var_gen = Gen::new();
    let mut comps =
        from_ast::ast_to_ir(ast, PassRunner::compile(), &mut ctx, &mut var_gen);

    // for comp in &comps {
    //     println!("{}", comp);
    // }

    let Some(main) = comps
        .iter_mut()
        .find(|comp| comp.label().name.unmangled() == "main")
    else {
        panic!("no `main` component")
    };

    println!("{}", main);

    // let mut agens = Vec::new();
    // for comp in comps
    //     .iter()
    //     .filter(|comp| comp.has_attribute(Attribute::Kernel))
    // {}

    log::info!("Emitting calyx accelerator and address generator (TODO)...");

    // let mut agen_backend = BackendBuilder::new()
    //     .target(PrintTarget)
    //     .through(AddressGeneratorTransform)
    //     .build();
    // agen_backend.emit(main, &mut ctx, &mut var_gen, OutputFile::Stdout)?;

    PassRunner::lower().run(main, &mut ctx);

    let mut calyx_backend = BackendBuilder::new().target(PrintTarget).build();
    calyx_backend.emit(main, &mut ctx, &mut var_gen, OutputFile::Stdout)?;

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
