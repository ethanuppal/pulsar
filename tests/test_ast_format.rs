#[cfg(test)]
mod tests {
    use insta::assert_snapshot;
    use pulsar::{
        frontend::{
            ast::{Expr, ExprValue, Node, NodeValue},
            token::{Token, TokenType},
            ty::{StmtType, Type}
        },
        utils::{loc::Loc, mutcell::MutCell}
    };

    fn make_token(ty: TokenType, value: &str) -> Token {
        Token {
            ty,
            value: value.into(),
            loc: Loc::default()
        }
    }

    fn dumb_token() -> Token {
        make_token(TokenType::Identifier, "".into())
    }

    #[test]
    fn test_format_constant_int() {
        assert_snapshot!(
            Expr { value: ExprValue::ConstantInt(5), ty: Type::make_unknown(), start: dumb_token()  }.to_string(),
            @"5"
        )
    }

    #[test]
    fn test_format_bin_op() {
        let left = Box::new(Expr {
            value: ExprValue::ConstantInt(5),
            ty: Type::make_unknown(),
            start: dumb_token()
        });
        let right = Box::new(Expr {
            value: ExprValue::ConstantInt(3),
            ty: Type::make_unknown(),
            start: dumb_token()
        });
        assert_snapshot!(
            Expr { value: ExprValue::BinOp(left, make_token(TokenType::Plus, "+"), right), ty: Type::make_unknown(), start: dumb_token() }.to_string(),
            @"(5 + 3)"
        )
    }

    #[test]
    fn test_format_function() {
        let body = vec![Node {
            value: NodeValue::LetBinding {
                name: Token {
                    ty: TokenType::Identifier,
                    value: "x".into(),
                    loc: Loc::default()
                },
                hint: None,
                value: Box::new(Expr {
                    value: ExprValue::ConstantInt(5),
                    ty: Type::make_unknown(),
                    start: dumb_token()
                })
            },
            ty: StmtType::make_unknown(),
            start_token: MutCell::new(None),
            end_token: MutCell::new(None)
        }];
        assert_snapshot!(
            Node {
                value: NodeValue::Function { name: Token {
                    ty: TokenType::Identifier,

                    value: "foo".into(),
                    loc: Loc::default()
                },
                pure_token: None,
                params: vec![],
                ret: Type::Unit,body: body.clone() },
                ty: StmtType::make_unknown(),
                start_token: MutCell::new(None),
                end_token: MutCell::new(None)
            }.to_string(),
            @r###"
        func foo() -> Unit {
            let x = 5
        }
        "###
        );

        assert_snapshot!(Node {
            value: NodeValue::Function {
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
            ty: StmtType::make_unknown(),
            start_token: MutCell::new(None),
            end_token: MutCell::new(None)
        }
        .to_string())
    }

    #[test]
    fn test_format_let_binding() {
        assert_snapshot!(
            Node { value: NodeValue::LetBinding { name: Token {
                ty: TokenType::Identifier,
                value: "x".into(),
                loc: Loc::default()
            },
            hint: None,
            value: Box::new(Expr { value: ExprValue::ConstantInt(5), ty: Type::make_unknown(), start: dumb_token() }) }, ty: StmtType::make_unknown(),
            start_token: MutCell::new(None),
            end_token: MutCell::new(None) }.to_string(),
            @"let x = 5"
        )
    }
}
