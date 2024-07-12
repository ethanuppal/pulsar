#[cfg(test)]
mod tests {
    use insta::assert_snapshot;
    use pulsar_frontend::{
        ast::{Expr, ExprValue, Stmt, StmtValue},
        token::{Token, TokenType},
        ty::Type
    };
    use pulsar_utils::loc::Loc;

    fn make_token(ty: TokenType, value: &str) -> Token {
        Token {
            ty,
            value: value.into(),
            loc: Loc::default()
        }
    }

    fn dumb_token() -> Token {
        make_token(TokenType::Identifier, "")
    }

    #[test]
    fn test_format_constant_int() {
        assert_snapshot!(
            Expr::new(ExprValue::ConstantInt(5), dumb_token(), dumb_token()).to_string(),
            @"5"
        )
    }

    #[test]
    fn test_format_bin_op() {
        let left = Box::new(Expr::new(
            ExprValue::ConstantInt(5),
            dumb_token(),
            dumb_token()
        ));
        let right = Box::new(Expr::new(
            ExprValue::ConstantInt(3),
            dumb_token(),
            dumb_token()
        ));
        assert_snapshot!(
            Expr::new(ExprValue::InfixBop(left, make_token(TokenType::Plus, "+"), right), dumb_token(), dumb_token()).to_string(),
            @"(5 + 3)"
        )
    }

    #[test]
    fn test_format_function() {
        let body = vec![Stmt::new(
            StmtValue::LetBinding {
                name: Token {
                    ty: TokenType::Identifier,
                    value: "x".into(),
                    loc: Loc::default()
                },
                hint: None,
                value: Box::new(Expr::new(
                    ExprValue::ConstantInt(5),
                    dumb_token(),
                    dumb_token()
                ))
            },
            dumb_token(),
            dumb_token()
        )];
        assert_snapshot!(
            Stmt::new(StmtValue::Function { name: Token {
                ty: TokenType::Identifier,
                value: "foo".into(),
                loc: Loc::default()
            },
            pure_token: None,
            params: vec![],
            ret: Type::Unit,body: body.clone() }, dumb_token(), dumb_token()).to_string(),
            @r###"
        func foo() -> Unit {
            let x = 5
        }
        "###
        );

        assert_snapshot!(Stmt::new(
            StmtValue::Function {
                name: Token {
                    ty: TokenType::Identifier,
                    value: "foo".into(),
                    loc: Loc::default()
                },
                params: vec![],
                ret: Type::Unit,
                pure_token: Some(make_token(TokenType::Pure, "pure")),
                body: body.clone()
            },
            dumb_token(),
            dumb_token()
        )
        .to_string())
    }

    #[test]
    fn test_format_let_binding() {
        assert_snapshot!(
            Stmt::new(StmtValue::LetBinding { name: Token {
                ty: TokenType::Identifier,
                value: "x".into(),
                loc: Loc::default()
            },
            hint: None,
            value: Box::new(Expr::new(ExprValue::ConstantInt(5), dumb_token(), dumb_token())) }, dumb_token(), dumb_token()).to_string(),
            @"let x = 5"
        )
    }
}
