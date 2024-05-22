#[cfg(test)]
mod tests {
    use insta::assert_snapshot;
    use pulsar::frontend::ast::{Expr, ExprValue, Node, NodeValue};
    use pulsar::frontend::token::{Token, TokenType};
    use pulsar::utils::loc::Loc;

    #[test]
    fn test_format_constant_int() {
        assert_snapshot!(
            Expr { value: ExprValue::ConstantInt(5), ty: None }.to_string(),
            @"5"
        )
    }

    #[test]
    fn test_format_bin_op() {
        let left = Box::new(Expr {
            value: ExprValue::ConstantInt(5),
            ty: None
        });
        let right = Box::new(Expr {
            value: ExprValue::ConstantInt(3),
            ty: None
        });
        assert_snapshot!(
            Expr { value: ExprValue::BinOp(left, TokenType::Plus, right), ty: None }.to_string(),
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
                value: Box::new(Expr {
                    value: ExprValue::ConstantInt(5),
                    ty: None
                })
            },
            ty: None
        }];
        assert_snapshot!(
            Node {
                value: NodeValue::Function { name: Token {
                    ty: TokenType::Identifier,
                    value: "foo".into(),
                    loc: Loc::default()
                }, body },
                ty: None
            }.to_string(),
            @r###"
        func foo() {
            let x = 5
        }
        "###
        )
    }

    #[test]
    fn test_format_let_binding() {
        assert_snapshot!(
            Node { value: NodeValue::LetBinding { name: Token {
                ty: TokenType::Identifier,
                value: "x".into(),
                loc: Loc::default()
            }, value: Box::new(Expr { value: ExprValue::ConstantInt(5), ty: None }) }, ty: None }.to_string(),
            @"let x = 5"
        )
    }
}
