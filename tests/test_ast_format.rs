//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

#[cfg(test)]
mod tests {
    use insta::assert_snapshot;
    use pulsar_frontend::{
        ast::{
            decl::{Decl, DeclValue}, expr::{Expr, ExprValue}, node::{AsNodePool, NodeInterface}, stmt::{Stmt, StmtValue}, ty::TypeValue
        },
        token::{Token, TokenType}
    };
    use pulsar_lang::context::Context;
    use pulsar_utils::{
        span::Loc,
        pool::{AsPool, Handle}
    };
    use std::cell::RefCell;

    thread_local! {
        static CTX: RefCell<Context> = RefCell::new(Context::new().unwrap());
    }

    fn make_token<C: AsMut<Context>>(
        mut ctx: C, ty: TokenType, value: &str
    ) -> Handle<Token> {
        ctx.as_mut().add(Token {
            ty,
            value: value.into(),
            loc: Loc::default()
        })
    }

    fn dumb_token<C: AsMut<Context>>(ctx: C) -> Handle<Token> {
        make_token(ctx, TokenType::Identifier, "")
    }

    #[test]
    fn test_format_constant_int() {
        let dumb = CTX.with_borrow_mut(|ctx| dumb_token(ctx));
        
        assert_snapshot!(
            Expr::new(ExprValue::ConstantInt(5), dumb,dumb).to_string(), @"5"
        )
    }

    #[test]
    fn test_format_bin_op() {
        let dumb = CTX.with_borrow_mut(|ctx| dumb_token(ctx));
        let plus =
            CTX.with_borrow_mut(|ctx| make_token(ctx, TokenType::Plus, "+"));
        CTX.with_borrow_mut(|ctx| {
            let left = ctx.add(Expr::new(
                ExprValue::ConstantInt(5),
                dumb,
                dumb
            ));
            let right = ctx.add(Expr::new(
                ExprValue::ConstantInt(3),
                dumb,
                dumb
            ));

            assert_snapshot!(
                Expr::new(ExprValue::InfixBop(left, plus, right), dumb, dumb).to_string(),
                @"(5 + 3)"
            )
        });
    }

    #[test]
    fn test_format_function() {
        let dumb = CTX.with_borrow_mut(|ctx| dumb_token(ctx));
        let x =
            CTX.with_borrow_mut(|ctx| make_token(ctx, TokenType::Identifier, "x"));

        CTX.with_borrow_mut(|ctx| {
            let value = ctx.new(
                ExprValue::ConstantInt(5),
                dumb,
                dumb
            );
            let body = vec![ctx.new(
                StmtValue::Let {
                    name: x,
                    hint: None,
                    value
                },
                dumb,
                dumb
            )];
            let ty = ctx.new(TypeValue::Int64, dumb, dumb);

            assert_snapshot!(
                Decl::new(DeclValue::Function { func: dumb, name: x, inputs: vec![(x, ty), (x, ty)], outputs: vec![(x, ty), (x, ty)], body }, dumb, dumb),         
            @r###"
            func x(x: Int64, x: Int64) -> (x: Int64, x: Int64) {
                let x = 5
            }
            "###
            );
        });
    }

    #[test]
    fn test_format_let_binding() {
        let dumb = CTX.with_borrow_mut(|ctx| dumb_token(ctx));
        let x = CTX.with_borrow_mut(|ctx| make_token(ctx, TokenType::Identifier, "x"));
        let value = CTX.with_borrow_mut(|ctx| {
            ctx.new(
                ExprValue::ConstantInt(5),
                dumb,
                dumb
            )
        });
    
        assert_snapshot!(
            Stmt::new(
                StmtValue::Let {
                    name: x,
                    hint: None,
                    value 
                },
                dumb,
                dumb
            ).to_string(),
            @"let x = 5"
        );
    }
}
