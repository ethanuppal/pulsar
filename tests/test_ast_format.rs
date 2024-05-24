#[cfg(test)]
mod tests {
    use insta::assert_snapshot;
    use pulsar::frontend::ast::{Expr, ExprValue, Node, NodeValue};
    use pulsar::frontend::token::{Token, TokenType};
    use pulsar::frontend::ty::{StmtType, Type};
    use pulsar::utils::loc::Loc;

    fn make_token(ty: TokenType, value: &str) -> Token {
        Token {
            ty,
            value: value.into(),
            loc: Loc::default()
        }
    }

    #[test]
    fn test_format_constant_int() {
        assert_snapshot!(
            Expr { value: ExprValue::ConstantInt(5), ty: Type::make_unknown() }.to_string(),
            @"5"
        )
    }

    #[test]
    fn test_format_bin_op() {
        let left = Box::new(Expr {
            value: ExprValue::ConstantInt(5),
            ty: Type::make_unknown()
        });
        let right = Box::new(Expr {
            value: ExprValue::ConstantInt(3),
            ty: Type::make_unknown()
        });
        assert_snapshot!(
            Expr { value: ExprValue::BinOp(left, make_token(TokenType::Plus, "+"), right), ty: Type::make_unknown() }.to_string(),
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
                    ty: Type::make_unknown()
                })
            },
            ty: StmtType::make_unknown()
        }];
        assert_snapshot!(
            Node {
                value: NodeValue::Function { name: Token {
                    ty: TokenType::Identifier,

                    value: "foo".into(),
                    loc: Loc::default()
                },
                is_pure: false,
                params: vec![],
                ret: Type::Unit,body: body.clone() },
                ty: StmtType::make_unknown()
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
                is_pure: true,
                body: body.clone()
            },
            ty: StmtType::make_unknown()
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
            }, hint: None, value: Box::new(Expr { value: ExprValue::ConstantInt(5), ty: Type::make_unknown() }) }, ty: StmtType::make_unknown() }.to_string(),
            @"let x = 5"
        )
    }
}
