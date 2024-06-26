// Copyright (C) 2024 Ethan Uppal. All rights reserved.
use super::{
    ast::{Expr, ExprValue, Node},
    op::{Op, Precedence},
    token::{Token, TokenType},
    ty::{Type, TypeCell}
};
use crate::{
    ast::{NodeValue, TokenRegionProvider},
    op::Associativity
};
use pulsar_utils::error::{
    Error, ErrorBuilder, ErrorCode, ErrorManager, Level, Style
};
use std::{cell::RefCell, fmt::Display, rc::Rc};

enum Ctx {
    In(String),
    Between(String),
    For(String),
    Begin(String),
    End(String),
    After(String)
}

impl Display for Ctx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::In(loc) => format!("in {}", loc),
            Self::Between(values) => format!("between {}", values),
            Self::For(purpose) => format!("for {}", purpose),
            Self::Begin(loc) => format!("at start of {}", loc),
            Self::End(loc) => format!("at end of {}", loc),
            Self::After(loc) => format!("after {}", loc)
        }
        .fmt(f)
    }
}

pub struct Parser {
    pos: usize,
    buffer: Vec<Token>,
    error_manager: Rc<RefCell<ErrorManager>>
}

impl Parser {
    /// Constructs a parser for the given token buffer `buffer`.
    pub fn new(
        buffer: Vec<Token>, error_manager: Rc<RefCell<ErrorManager>>
    ) -> Parser {
        Parser {
            pos: 0,
            buffer,
            error_manager
        }
    }

    fn is_eof(&self) -> bool {
        self.pos == self.buffer.len()
    }

    fn previous(&self) -> &Token {
        &self.buffer[self.pos - 1]
    }

    fn current(&self) -> &Token {
        &self.buffer[self.pos]
    }

    fn current_opt(&self) -> Option<&Token> {
        if self.is_eof() {
            None
        } else {
            Some(self.current())
        }
    }

    /// Parses the current token as an operator without consuming any tokens.
    fn current_op_opt(&self) -> Option<Op> {
        self.current_opt().and_then(|token| Op::from(token.ty))
    }

    fn is_at(&self, ty: TokenType) -> bool {
        self.current_opt()
            .map(|token| token.ty == ty)
            .unwrap_or_default()
    }

    fn next_is(&self, ty: TokenType) -> bool {
        if self.pos + 1 < self.buffer.len() {
            self.buffer[self.pos + 1].ty == ty
        } else {
            false
        }
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn take(&mut self) -> Token {
        let result = self.current().clone();
        self.advance();
        result
    }

    /// Reverts a [`Parser::advance`], returning `true` unless `self.is_eof()`.
    fn unget(&mut self) -> bool {
        if self.pos > 0 {
            self.pos -= 1;
            true
        } else {
            false
        }
    }

    /// Skips past tokens that can be arbitrarily inserted, such as newlines.
    fn consume_ignored(&mut self) {
        while !self.is_eof() && self.current().ty == TokenType::Newline {
            self.advance()
        }
    }

    /// Returns the next token in the stream if it is of type `token_type` and
    /// reports an error otherwise, returning `None`.
    fn expect(&mut self, token_type: TokenType, context: Ctx) -> Option<Token> {
        if self.is_eof() {
            self.report_unexpected_eof(context);
            None
        } else if self.current().ty != token_type {
            self.report_expected_token(
                token_type,
                &self.current().clone(),
                &context.to_string()
            );
            None
        } else {
            Some(self.take())
        }
    }

    /// [Reports](Parser::report): EOF is unexpectedly encountered in the
    /// parsing context `context`.
    ///
    /// Requires: `!buffer.is_empty()`.
    fn report_unexpected_eof(&mut self, context: Ctx) {
        self.report(
            ErrorBuilder::new()
                .of_style(Style::Primary)
                .at_level(Level::Error)
                .with_code(ErrorCode::UnexpectedEOF)
                .at_region(self.buffer.last().unwrap())
                .explain(format!("Unexpected EOF {}", context))
                .build()
        );
    }

    /// [Reports](Parser::report): The token type of the found token `actual`
    /// diverges from the expected type `expected_ty` in the parsing context
    /// `context`.
    fn report_expected_token(
        &mut self, expected_ty: TokenType, actual: &Token, context: &str
    ) {
        self.report(
            ErrorBuilder::new()
                .of_style(Style::Primary)
                .at_level(Level::Error)
                .with_code(ErrorCode::UnexpectedToken)
                .at_region(actual)
                .message(format!("Expected '{:?}' {}", expected_ty, context))
                .explain(format!("Received '{:?}' here", actual.ty))
                .build()
        );
    }

    /// [Reports](Parser::report): See [`Parser::report_expected_token`].
    fn report_expected_tokens(
        &mut self, expected_tys: &[TokenType], actual: &Token, context: Ctx
    ) {
        self.report(
            ErrorBuilder::new()
                .of_style(Style::Primary)
                .at_level(Level::Error)
                .with_code(ErrorCode::UnexpectedToken)
                .at_region(actual)
                .message(format!(
                    "Expected one of {} {}",
                    expected_tys
                        .iter()
                        .map(|ty| format!("'{:?}'", ty))
                        .collect::<Vec<String>>()
                        .join(", "),
                    context
                ))
                .explain(format!("Received '{:?}' here", actual.ty))
                .build()
        );
    }

    /// Constructs a secondary-style error that refers back to a previous token
    /// `refback` with additional explanation `explain`.
    fn report_refback(&mut self, refback: &Token, explain: String) {
        self.report(
            ErrorBuilder::new()
                .of_style(Style::Secondary)
                .at_level(Level::Error)
                .at_region(refback)
                .continues()
                .explain(explain)
                .build()
        );
    }

    /// [Reports](Parser::report): A construct (marked by `token`) is found at
    /// top level that should not be. See [`Parser::report_not_top_level`].
    fn report_top_level(&mut self, token: &Token) {
        self.report(ErrorBuilder::new()
            .of_style(Style::Primary)
            .at_level(Level::Error)
            .with_code(ErrorCode::InvalidTopLevelConstruct)
            .at_region(token)
            .message(format!("Unexpected {:?} at top level", token.ty))
            .fix(
                "Allowed constructs at top level include functions and imports"
                    .into()
            )
            .build());
    }

    /// [Reports](Parser::report): A construct (marked by `token`) that belongs
    /// only at top level is found further nested. See
    /// [`Parser::report_top_level`].
    fn report_not_top_level(&mut self, token: &Token) {
        self.report(
            ErrorBuilder::new()
                .of_style(Style::Primary)
                .at_level(Level::Error)
                .with_code(ErrorCode::ConstructShouldBeTopLevel)
                .at_region(token)
                .message("Unexpected top-level construct".into())
                .fix("Did you mean to place it at the top level?".into())
                .build()
        );
    }

    /// [Reports](Parser::report): `token` represents an invalid start to a
    /// statement.
    fn report_invalid_token(&mut self, token: &Token) {
        self.report(
            ErrorBuilder::new()
                .of_style(Style::Primary)
                .at_level(Level::Error)
                .with_code(ErrorCode::InvalidTokenForStatement)
                .at_region(token)
                .message("Invalid token at the start of a statement".into())
                .build()
        );
    }

    /// [Reports](Parser::report): `token` was used incorrectly as a `usage`
    /// operator when it is not.
    fn report_invalid_operator(&mut self, token: &Token, usage: &str) {
        self.report(
            ErrorBuilder::new()
                .of_style(Style::Primary)
                .at_level(Level::Error)
                .with_code(ErrorCode::InvalidOperatorSyntax)
                .at_region(token)
                .message(format!(
                    "{} is not an {} operator",
                    token.value, usage
                ))
                .explain(format!("Used here as an {} operator", usage))
                .build()
        );
    }

    fn report(&mut self, error: Error) {
        self.error_manager.borrow_mut().record(error);
    }
}

macro_rules! expect_n {
    ($self:ident in [$($token_type:expr),*] => $context:expr) => {
        if $self.is_eof() {
            $self.report_unexpected_eof($context.into());
            None
        } else if ![$($token_type),*].contains(&$self.current().ty) {
            $self.report_expected_tokens(
                &[$($token_type),*],
                &$self.current().clone(),
                $context
            );
            None
        } else {
            Some($self.take())
        }
    };
}

/// `contained_in! { self; left_type, name, right_type; ... }` computes an
/// expression or series of statements followed by an expression (`...`)
/// surrounded by tokens of type `left_type` and `right_type`.
macro_rules! contained_in {
    ($self:ident; $open_type:expr, $loc_ctx:expr, $close_type:expr; $($action:tt)*) => {
        {
            let open_token = $self.expect($open_type, Ctx::Begin($loc_ctx.into()))?;
            let result = {$($action)*};
            let close_token = $self.expect($close_type, Ctx::End($loc_ctx.into()));
            if close_token.is_none() {
                $self.report_refback(
                    &open_token,
                    format!("{} opened here", $open_type)
                );
                return None;
            }
            Some(result)
        }
    };
}

/// See [`parse_full_expr!`] and [`parse_full_node!`].
macro_rules! parse_full_abstract {
    ($self:ident.$method:ident<$type:ty>($($arg:expr),*)) => {{
        let start_token = $self.current().clone();
        let value = $self.$method($($arg),*);
        let end_token = $self.previous().clone();
        value.map(|v| <$type>::new(v, start_token, end_token))
    }};
}

/// `parse_full_expr!(self.method_returning_expr_value())` wraps the
/// [`ExprValue`] as an [`Expr`] by keeping track of the surrounding tokens.
macro_rules! parse_full_expr {
    ($self:ident.$method:ident($($arg:expr),*)) => {
        parse_full_abstract!($self.$method<Expr>($($arg),*))
    };
}

/// `parse_full_node!(self.method_returning_node_value())` wraps the
/// [`NodeValue`] as an [`Node`] by keeping track of the surrounding tokens.
macro_rules! parse_full_node {
    ($self:ident.$method:ident($($arg:expr),*)) => {
        parse_full_abstract!($self.$method<Node>($($arg),*))
    };
}

impl TokenType {
    fn begins_top_level_construct(&self) -> bool {
        matches!(self, Self::Func | Self::Pure) // || Self::Import
    }
}

impl Parser {
    /// Advances until EOF, or when specified by `current_exit`, or when a
    /// top-level construct is potentially found.
    fn synchronize(
        &mut self, custom_exit: fn(&Token) -> bool, description: String
    ) {
        if !self.is_eof() {
            self.report(
                ErrorBuilder::new()
                    .of_style(Style::Primary)
                    .at_level(Level::Info)
                    .with_code(ErrorCode::UnexpectedToken)
                    .at_region(self.current())
                    .message(
                        "Attempting to recover understanding of code".into()
                    )
                    .explain(description)
                    .build()
            );
        }
        while !self.is_eof()
            && !custom_exit(self.current())
            && !self.current().ty.begins_top_level_construct()
        {
            self.advance();
        }
    }

    /// Identical to [`Parser::synchronize`] but with no custom exit.`
    fn attempt_restore_to_top_level(&mut self) {
        self.synchronize(|_| false, "Seeking top-level construct".into());
    }

    fn parse_primary_type(&mut self, name: Option<&str>) -> Option<Type> {
        let context = match name {
            Some(name) => Ctx::In(name.into()),
            None => Ctx::For("primary type".into())
        };
        let type_token = self.expect(TokenType::Identifier, context)?;
        Some(match type_token.value.as_str() {
            "Int64" | "Int" => Type::Int64,
            "Unit" => Type::Unit,
            other => Type::Name(other.into())
        })
    }

    fn parse_array_type(&mut self, inner: Type) -> Option<Type> {
        let size_token = contained_in! { self;
            TokenType::LeftBracket, "array type", TokenType::RightBracket;
            self.expect(TokenType::Integer, Ctx::For("array size".into()))?
        }?;

        let size = size_token
            .value
            .as_str()
            .parse::<i64>()
            .expect("number token can be parsed as number");
        if size < 0 {
            self.report(
                ErrorBuilder::new()
                    .of_style(Style::Primary)
                    .at_level(Level::Error)
                    .with_code(ErrorCode::MalformedType)
                    .at_region(&size_token)
                    .message("Array size cannot be negative".into())
                    .build()
            );
            return None;
        } else if size == 0 {
            self.report(
                ErrorBuilder::new()
                    .of_style(Style::Primary)
                    .at_level(Level::Warning)
                    .with_code(ErrorCode::MalformedType)
                    .at_region(&size_token)
                    .message("Array size is zero".into())
                    .build()
            );
        }

        let result = Type::Array(TypeCell::new(inner), size as isize);
        if self.is_at(TokenType::LeftBracket) {
            self.parse_array_type(result)
        } else {
            Some(result)
        }
    }

    fn parse_type(&mut self, name: Option<&str>) -> Option<Type> {
        if self.is_eof() {
            self.report_unexpected_eof(Ctx::In(
                match name {
                    Some(name) => name,
                    None => "type"
                }
                .into()
            ));
            return None;
        }
        let primary = self.parse_primary_type(name)?;
        if self.is_at(TokenType::LeftBracket) {
            self.parse_array_type(primary)
        } else {
            Some(primary)
        }
    }

    fn parse_array_literal_expr_value(&mut self) -> Option<ExprValue> {
        let mut elements = vec![];
        let mut should_continue = false;
        let mut i = 0;
        contained_in! { self;
            TokenType::LeftBracket, "array literal", TokenType::RightBracket;

            while !self.is_eof() && self.current().ty != TokenType::RightBracket {
                if i > 0 {
                    self.expect(
                        TokenType::Comma,
                        Ctx::Between("array elements".into())
                    )?;
                    self.consume_ignored();
                }
                match (self.current_opt().map(|token| token.ty), i) {
                    (Some(TokenType::RightBracket), i) => {
                        if i > 0 {
                            break;
                        }
                    }
                    (Some(TokenType::Dots), _) => {
                        should_continue = true;
                        self.advance();
                        break;
                    }
                    _ => {}
                }

                let element_opt = self.parse_expr();
                if let Some(element) = element_opt {
                    elements.push(element);
                } else {
                    self.synchronize(|token| token.ty == TokenType::RightBrace, "Seeking end of array literal".into());
                    return None;
                }

                i += 1;
            }
        };

        Some(ExprValue::ArrayLiteral(elements, should_continue))
    }

    fn parse_literal_expr_value(&mut self) -> Option<ExprValue> {
        let literal_token = expect_n! { self in
            [TokenType::Integer, TokenType::Float, TokenType::Char, TokenType::LeftBracket, TokenType::Identifier] => Ctx::Begin("literal expression".into())
        }?;
        match literal_token.ty {
            TokenType::Integer => Some(ExprValue::ConstantInt(
                literal_token.value.parse::<i64>().unwrap()
            )),
            TokenType::LeftBracket => {
                self.unget();
                self.parse_array_literal_expr_value()
            }
            TokenType::Identifier => {
                Some(ExprValue::BoundName(literal_token.clone()))
            }
            _ => None
        }
    }

    fn parse_unary_prefix_expr_value(
        &mut self, prefix_op: Op
    ) -> Option<ExprValue> {
        if prefix_op.is_unary_prefix() {
            self.report_invalid_operator(&self.current().clone(), "unary");
            return None;
        }

        let op_token = self.take();
        let rhs = self.parse_expr()?;

        Some(ExprValue::PrefixOp(op_token, Box::new(rhs)))
    }

    fn parse_postfix_binary_expr_value(
        &mut self, mut lhs: Expr
    ) -> Option<ExprValue> {
        while let Some(postfix_bop) =
            self.current_op_opt().and_then(|op| op.postfix_binary)
        {
            let op1 = self.take();
            let rhs = self.parse_expr()?;
            let op2 = self.expect(
                postfix_bop.close_token_ty,
                postfix_bop.name.map_or(
                    Ctx::After(
                        "second subexpression in postfix binary expression"
                            .into()
                    ),
                    |name| Ctx::In(name)
                )
            )?;
            let start_token = lhs.start_token().clone();
            let end_token = op2.clone();
            lhs = Expr::new(
                ExprValue::PostfixBop(Box::new(lhs), op1, Box::new(rhs), op2),
                start_token,
                end_token
            );
        }
        Some(lhs.value)
    }

    /// Warning: do not call this function unless it is wrapped in
    /// [`parse_expr_full!`].
    fn parse_call_expr_value(&mut self) -> Option<ExprValue> {
        let name = self.expect(
            TokenType::Identifier,
            Ctx::Begin("call expression".into())
        )?;

        let mut args = vec![];
        contained_in! { self;
            TokenType::LeftPar, "call expression", TokenType::RightPar;
            let mut i = 0;
            while !self.is_eof() && self.current().ty != TokenType::RightPar {
                if i > 0 {
                    self.expect(TokenType::Comma, Ctx::Between("arguments".into()))?;
                    self.consume_ignored();
                }
                if self.is_at(TokenType::RightPar) {
                    break;
                }

                let arg_opt = self.parse_expr();
                if let Some(arg) = arg_opt {
                    args.push(arg);
                } else {
                    self.synchronize(|token| token.ty == TokenType::RightPar, "Seeking end of call arguments".into());
                    return None;
                }

                i += 1;
            }
        };

        Some(ExprValue::Call(name.clone(), args))
    }

    fn parse_primary_expr_value_aux(&mut self) -> Option<ExprValue> {
        if self.is_eof() {
            self.report_unexpected_eof(Ctx::Begin("primary expression".into()));
            None
        } else if let Some(prefix_op) =
            self.current_op_opt().filter(|op| op.is_unary_prefix())
        {
            self.parse_unary_prefix_expr_value(prefix_op)
        } else if self.is_at(TokenType::LeftPar) {
            let open_paren = self.take();
            let expr_value = self.parse_expr()?.value;
            let closing_paren =
                self.expect(TokenType::RightPar, Ctx::In("expression".into()));
            if closing_paren.is_none() {
                self.report_refback(
                    &open_paren,
                    "Parentheses opened here".into()
                );
                None
            } else {
                Some(expr_value)
            }
        } else if self.is_at(TokenType::HardwareMap) {
            let map_token = self.expect(
                TokenType::HardwareMap,
                Ctx::Begin("hardware map".into())
            )?;
            self.expect(
                TokenType::LeftAngle,
                Ctx::In("hardware map expression".into())
            )?;
            let parallel_factor_token = self.expect(
                TokenType::Integer,
                Ctx::In("hardware map expression".into())
            )?;
            self.expect(
                TokenType::RightAngle,
                Ctx::In("hardware map expression".into())
            )?;
            self.expect(
                TokenType::LeftPar,
                Ctx::In("hardware map expression".into())
            )?;
            let f = self.expect(
                TokenType::Identifier,
                Ctx::In("hardware map expression".into())
            )?;
            self.expect(
                TokenType::Comma,
                Ctx::In("hardware map expression".into())
            )?;
            let arr = self.parse_expr()?;
            self.expect(
                TokenType::RightPar,
                Ctx::In("hardware map expression".into())
            )?;
            // TODO: check for negatives
            Some(ExprValue::HardwareMap(
                map_token.clone(),
                parallel_factor_token.value.parse::<usize>().unwrap(),
                f,
                Box::new(arr)
            ))
        } else if self.is_at(TokenType::Identifier)
            && self.next_is(TokenType::LeftPar)
        {
            // TODO: allow calling expressions and more complex names with `::`
            self.parse_call_expr_value()
        } else {
            self.parse_literal_expr_value()
        }
    }

    fn parse_primary_expr_value(&mut self) -> Option<ExprValue> {
        let primary = parse_full_expr!(self.parse_primary_expr_value_aux())?;
        if self
            .current_op_opt()
            .map_or(false, |op| op.is_postfix_binary())
        {
            self.parse_postfix_binary_expr_value(primary)
        } else {
            Some(primary.value)
        }
    }

    /// Implements [operator-precedence parsing](https://en.wikipedia.org/wiki/Operator-precedence_parser).
    fn parse_infix_binary_expr(
        &mut self, mut lhs: Expr, min_precedence: Precedence
    ) -> Option<Expr> {
        let mut lookahead = self.current().clone();
        while !self.is_eof()
            && Op::from(lookahead.ty)
                .and_then(|op| op.infix_binary)
                .map(|bop| bop.precedence >= min_precedence)
                .unwrap_or_default()
        {
            let op_token = self.take();
            let bop = Op::from(op_token.ty)
                .and_then(|op| op.infix_binary)
                .expect("while cond guarantees");

            let mut rhs = parse_full_expr!(self.parse_primary_expr_value())?;
            if self.is_eof() {
                break;
            }
            lookahead = self.current().clone();
            while !self.is_eof()
                && Op::from(lookahead.ty)
                    .and_then(|next_op| next_op.infix_binary)
                    .map(|next_bop| {
                        (next_bop.associativity == Associativity::Left
                            && next_bop.precedence > bop.precedence)
                            || (next_bop.associativity == Associativity::Right
                                && next_bop.precedence == bop.precedence)
                    })
                    .unwrap_or_default()
            {
                let next_bop = Op::from(lookahead.ty)
                    .and_then(|bop| bop.infix_binary)
                    .unwrap();

                let new_min_precedence = bop.precedence
                    + if next_bop.precedence > bop.precedence {
                        1
                    } else {
                        0
                    };
                rhs = self.parse_infix_binary_expr(rhs, new_min_precedence)?;
                lookahead = self.current().clone();
            }
            let start_token = lhs.start_token().clone();
            let end_token = rhs.end_token().clone();
            lhs = Expr::new(
                ExprValue::InfixBop(Box::new(lhs), op_token, Box::new(rhs)),
                start_token,
                end_token
            );
        }
        Some(lhs)
    }

    fn parse_expr(&mut self) -> Option<Expr> {
        self.consume_ignored();
        let primary = parse_full_expr!(self.parse_primary_expr_value())?;
        if let Some(op) = self.current_op_opt() {
            if op.is_infix_binary() {
                self.parse_infix_binary_expr(primary, -1)
            } else {
                self.report_invalid_operator(&self.current().clone(), "binary");
                None
            }
        } else {
            Some(primary)
        }
    }

    fn parse_let(&mut self) -> Option<NodeValue> {
        self.expect(TokenType::Let, Ctx::Begin("let binding".into()))?;

        let name = self.expect(
            TokenType::Identifier,
            Ctx::For("name in let binding".into())
        )?;

        let mut hint = None;
        if self.current().ty == TokenType::Colon {
            self.advance();
            hint = Some(TypeCell::new(
                self.parse_type("let binding type hint".into())?
            ));
        }

        self.expect(
            TokenType::Assign,
            Ctx::After("name in let binding".into())
        )?;

        let value = self.parse_expr()?;

        Some(NodeValue::LetBinding {
            name,
            hint,
            value: Box::new(value)
        })
    }

    fn parse_return(&mut self) -> Option<NodeValue> {
        let token = self
            .expect(TokenType::Return, Ctx::Begin("return statement".into()))?;

        let value = if self.is_at(TokenType::Newline) {
            None
        } else {
            Some(Box::new(self.parse_expr()?))
        };

        Some(NodeValue::Return {
            keyword_token: token,
            value
        })
    }

    /// Parses a brace-enclosed list of statements, e.g., `parse_block("function
    /// body")`.
    fn parse_block(&mut self, name: &str) -> Option<Vec<Node>> {
        self.consume_ignored();

        let mut nodes = vec![];
        let mut block_failed = false;

        contained_in! { self;
            TokenType::LeftBrace, name, TokenType::RightBrace;
            self.consume_ignored();
            while !self.is_eof() && self.current().ty != TokenType::RightBrace {
                let stmt_opt = self.parse_stmt(false);
                if let Some(stmt) = stmt_opt {
                    nodes.push(stmt);
                } else {
                    block_failed = true;
                    self.synchronize(|token| token.ty == TokenType::RightBrace, format!("Seeking end of {}", name));
                    break;
                }
            }
        };

        if block_failed {
            None
        } else {
            Some(nodes)
        }
    }

    fn parse_func(&mut self) -> Option<NodeValue> {
        let mut pure_token = None;
        if self.is_at(TokenType::Pure) {
            pure_token = Some(self.take());
        }

        self.expect(
            TokenType::Func,
            Ctx::Begin("function declaration".into())
        )?;

        let name = self
            .expect(TokenType::Identifier, Ctx::For("function name".into()))?;

        let open_paren = self.expect(
            TokenType::LeftPar,
            Ctx::Begin(format!("function parameters in `{}`", name.value))
        )?;

        self.consume_ignored();

        let mut i = 0;
        let mut params = vec![];
        while !self.is_eof() && self.current().ty != TokenType::RightPar {
            if i > 0 {
                self.expect(
                    TokenType::Comma,
                    Ctx::Between(format!(
                        "function parameters in `{}`",
                        name.value
                    ))
                )?;
                self.consume_ignored();
            }
            if self.is_at(TokenType::RightPar) {
                break;
            }

            let name = self.expect(
                TokenType::Identifier,
                Ctx::For(format!("parameter name in `{}`", name.value))
            )?;
            self.expect(
                TokenType::Colon,
                Ctx::After(format!("parameter name in `{}`", name.value))
            )?;
            let ty = self.parse_type("parameter type".into())?;
            params.push((name, ty));

            self.consume_ignored();
            i += 1
        }

        let close_paren = self.expect(
            TokenType::RightPar,
            Ctx::End("function parameters".into())
        );
        if close_paren.is_none() {
            self.report_refback(&open_paren, "Parentheses opened here".into());
            return None;
        }

        let mut ret = Type::Unit;
        if self.is_at(TokenType::Arrow) {
            self.advance();
            ret = self.parse_type("function return type".into())?;
        }

        let mut body = self.parse_block("function body")?;
        if ret == Type::Unit {
            body.push(
                Node::new(
                    NodeValue::Return {
                        keyword_token: name.clone(),
                        value: None
                    },
                    name.clone(),
                    name.clone()
                )
                .mark_generated()
            );
        }

        Some(NodeValue::Function {
            name: name.clone(),
            params,
            ret,
            pure_token,
            body
        })
    }

    /// Requires: a statement must have been parsed, and as consequence at least
    /// one token has already been encountered.
    fn end_stmt(&mut self) -> Option<Token> {
        let ending_token = self.previous().clone();

        if !self.is_eof() && self.current().ty == TokenType::RightBrace {
            return Some(ending_token);
        }

        self.expect(TokenType::Newline, Ctx::After("statement".into()))?;

        self.consume_ignored();

        Some(ending_token)
    }

    /// Requires: `!self.is_eof()`.
    fn parse_stmt_value(&mut self, top_level: bool) -> Option<NodeValue> {
        self.consume_ignored();
        if self.is_eof() || self.error_manager.borrow().is_full() {
            return None;
        }

        let current_ty = self.current().ty;
        match (current_ty, top_level) {
            (TokenType::Func, true) | (TokenType::Pure, true) => {
                if let Some(func) = self.parse_func() {
                    return Some(func);
                }
            }
            (TokenType::Let, false) => {
                if let Some(let_stmt) = self.parse_let() {
                    return Some(let_stmt);
                }
            }
            (TokenType::Return, false) => {
                if let Some(return_stmt) = self.parse_return() {
                    return Some(return_stmt);
                }
            }
            _ => {
                if current_ty.begins_top_level_construct() && !top_level {
                    self.report_not_top_level(&self.current().clone());
                } else if !current_ty.begins_top_level_construct() && top_level
                {
                    self.report_top_level(&self.current().clone());
                } else {
                    self.report_invalid_token(&self.current().clone());
                }
                self.advance();
            }
        }

        if top_level {
            self.attempt_restore_to_top_level();
            self.parse_stmt_value(true)
        } else {
            None
        }
    }

    fn parse_stmt(&mut self, top_level: bool) -> Option<Node> {
        if self.is_eof() {
            return None;
        }
        let node = parse_full_node!(self.parse_stmt_value(top_level))?;
        if self.end_stmt().is_none() {
            self.advance();
            return None;
        }
        Some(node)
    }
}

impl Iterator for Parser {
    type Item = Node;

    fn next(&mut self) -> Option<Node> {
        self.parse_stmt(true)
    }
}
