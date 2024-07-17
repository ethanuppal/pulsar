//! Implements a recursive-descent predictive parser. See the documentation at
//! [`Parser`].
//!
//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

use super::{
    op::{Op, Precedence},
    token::{Token, TokenType}
};
use crate::{
    ast::{
        decl::{Decl, DeclValue, ParamVec},
        expr::{Expr, ExprValue},
        node::NodeInterface,
        stmt::{Stmt, StmtValue},
        ty::{LiquidTypeValue, Type, TypeValue},
        AsASTPool, AST
    },
    op::Associativity
};
use pulsar_utils::{
    error::{Error, ErrorBuilder, ErrorCode, ErrorManager, Level, Style},
    pool::{Handle, HandleArray}
};
use std::{
    cmp,
    fmt::{self, Display}
};

pub type Block = (Handle<Token>, Vec<Handle<Stmt>>, Handle<Token>);

#[derive(Default)]
struct ParseErrorContext {
    loc: String,
    fix: Option<String>
}

impl ParseErrorContext {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn fix<S: AsRef<str>>(mut self, msg: S) -> Self {
        self.fix = Some(msg.as_ref().to_string());
        self
    }
}

macro_rules! where_constructor {
    ($name:ident, $constr:ident, $loc_str:expr) => {
        pub fn $name<S: AsRef<str>>(mut self, value: S) -> Self {
            self.loc = format!("{} {}", $loc_str, value.as_ref());
            self
        }
    };
}

impl ParseErrorContext {
    where_constructor!(in_, In, "in");
    where_constructor!(between, Between, "between");
    where_constructor!(for_, For, "for");
    where_constructor!(begin, Begin, "at start of");
    where_constructor!(end, End, "at end of");
    where_constructor!(after, After, "after");
}

/// The parser is constructed via helper, error reporting, and parse functions.
/// The naming convention is that functions named `parse_{decl,stmt,expr,type}`
/// are the driver functions for their given syntactic category (returning
/// [`crate::ast::node::Node`]s). These are merely wrappers around
/// `parse_{decl,stmt,expr,type}_value` functions, which drive individual
/// `parse_foo` functions (all of which return `Node::Value`s). At any point, a
/// `Node` can be constructed from its `Node::Value` by wrapping it in
/// [`parse_full_node!`].
///
/// Helpers:
///
/// - [`is_eof`](`Parser::is_eof`)
/// - [`previous`](`Parser::previous`)
/// - [`current`](`Parser::current`)
/// - [`current_opt`](`Parser::current_opt`)
/// - [`current_op_opt`](`Parser::current_op_opt`)
/// - [`is_at`](`Parser::is_at`)
/// - [`next_is`](`Parser::next_is`)
/// - [`advance`](`Parser::advance`)
/// - [`take`](`Parser::take`)
/// - [`unget`](`Parser::unget`)
/// - [`consume_ignored`](`Parser::consume_ignored`)
/// - [`expect`](`Parser::expect`)
/// - [`contained_in!`]
/// - [`parse_full_node!`]
/// - [`synchronize`](`Parser::synchronize`)
/// - [`attempt_restore_to_top_level`](`Parser::attempt_restore_to_top_level`)
///
/// Error Reporting:
///
/// - All the `report_*` functions as well as [`Parser::report`].
///
/// Parse:
///
/// - Types
///     - [`parse_primary_type`](Parser::parse_primary_type)
///     - [`parse_array_type`](Parser::parse_array_type)
///     - [`parse_type`](Parser::parse_type)
/// - Expressions
///     - [`parse_array_literal_expr_value`](Parser::parse_array_literal_expr_value)
///     - [`parse_literal_expr_value`](Parser::parse_literal_expr_value)
///     - [`parse_unary_prefix_expr_value`](Parser::parse_unary_prefix_expr_value)
///     - [`parse_postfix_binary_expr_value`](Parser::parse_postfix_binary_expr_value)
///     - [`parse_call_expr_value`](Parser::parse_call_expr_value)
///     - [`parse_primary_expr_value_aux`](Parser::parse_primary_expr_value_aux)
///     - [`parse_primary_expr_value`](Parser::parse_primary_expr_value)
///     - [`parse_infix_binary_expr`](Parser::parse_infix_binary_expr)
///     - [`parse_expr`](Parser::parse_expr)
/// - Statements
///     - [`parse_let`](Parser::parse_let)
///     - [`parse_block`](Parser::parse_block)
///     - [`end_stmt`](Parser::end_stmt)
///     - [`parse_stmt_value`](Parser::parse_stmt_value)
///     - [`parse_stmt`](Parser::parse_stmt)
/// - Declarations
///     - [`parse_params`](Parser::parse_params)
///     - [`parse_func`](Parser::parse_func)
///     - [`parse_decl_value`](Parser::parse_decl_value)
///     - [`parse_decl`](Parser::parse_decl)
pub struct Parser<'ast, 'err, P: AsASTPool> {
    pos: usize,
    buffer: HandleArray<Token>,
    ast_pool: &'ast mut P,
    error_manager: &'err mut ErrorManager
}

impl<'ast, 'err, P: AsASTPool> Parser<'ast, 'err, P> {
    /// Constructs a parser for the given token buffer `buffer`.
    pub fn new(
        buffer: HandleArray<Token>, ast_pool: &'ast mut P,
        error_manager: &'err mut ErrorManager
    ) -> Self {
        Self {
            pos: 0,
            buffer,
            ast_pool,
            error_manager
        }
    }

    fn is_eof(&self) -> bool {
        self.pos == self.buffer.len()
    }

    fn previous(&self) -> Handle<Token> {
        self.buffer.at(self.pos - 1)
    }

    fn current(&self) -> Handle<Token> {
        self.buffer.at(self.pos)
    }

    fn current_opt(&self) -> Option<Handle<Token>> {
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
            self.buffer.at(self.pos + 1).ty == ty
        } else {
            false
        }
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn take(&mut self) -> Handle<Token> {
        let result = self.current();
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
    fn expect(
        &mut self, token_type: TokenType, context: ParseErrorContext
    ) -> Option<Handle<Token>> {
        if self.is_eof() {
            self.report_unexpected_eof(context);
            None
        } else if self.current().ty != token_type {
            self.report_expected_token(token_type, self.current(), context);
            None
        } else {
            Some(self.take())
        }
    }

    /// [Reports](Parser::report): EOF is unexpectedly encountered in the
    /// parsing context `context`.
    ///
    /// Requires: `!buffer.is_empty()`.
    fn report_unexpected_eof(&mut self, context: ParseErrorContext) {
        self.report(
            ErrorBuilder::new()
                .of_style(Style::Primary)
                .at_level(Level::Error)
                .with_code(ErrorCode::UnexpectedEOF)
                .span(self.buffer.last().unwrap())
                .explain(format!("Unexpected EOF {}", context.loc))
                .maybe_fix(context.fix)
                .build()
        );
    }

    /// [Reports](Parser::report): The token type of the found token `actual`
    /// diverges from the expected type `expected_ty` in the parsing context
    /// `context`.
    fn report_expected_token(
        &mut self, expected_ty: TokenType, actual: Handle<Token>, context: &str
    ) {
        self.report(
            ErrorBuilder::new()
                .of_style(Style::Primary)
                .at_level(Level::Error)
                .with_code(ErrorCode::UnexpectedToken)
                .span(actual)
                .message(format!("Expected '{:?}' {}", expected_ty, context))
                .explain(format!("Received '{:?}' here", actual.ty))
                .build()
        );
    }

    /// [Reports](Parser::report): See [`Parser::report_expected_token`].
    fn report_expected_tokens(
        &mut self, expected_tys: &[TokenType], actual: Handle<Token>,
        context: ParseErrorContext
    ) {
        self.report(
            ErrorBuilder::new()
                .of_style(Style::Primary)
                .at_level(Level::Error)
                .with_code(ErrorCode::UnexpectedToken)
                .span(actual)
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
    fn report_refback(&mut self, refback: Handle<Token>, explain: String) {
        self.report(
            ErrorBuilder::new()
                .of_style(Style::Secondary)
                .at_level(Level::Error)
                .span(refback)
                .continues()
                .explain(explain)
                .build()
        );
    }

    /// [Reports](Parser::report): A construct (marked by `token`) is found at
    /// top level that should not be. See [`Parser::report_not_top_level`].
    fn report_invalid_top_level(&mut self, token: Handle<Token>) {
        self.report(ErrorBuilder::new()
            .of_style(Style::Primary)
            .at_level(Level::Error)
            .with_code(ErrorCode::InvalidTopLevelConstruct)
            .span(token)
            .message(format!("Unexpected {:?} at top level", token.ty))
            .fix(
                "Allowed constructs at top level include functions and imports"
                    .into()
            )
            .build());
    }

    /// [Reports](Parser::report): A construct (marked by `token`) that belongs
    /// only at top level is found further nested. See
    /// [`Parser::report_invalid_top_level`].
    fn report_unexpected_top_level(&mut self, token: Handle<Token>) {
        self.report(
            ErrorBuilder::new()
                .of_style(Style::Primary)
                .at_level(Level::Error)
                .with_code(ErrorCode::ConstructShouldBeTopLevel)
                .span(token)
                .message("Unexpected top-level construct".into())
                .fix("Did you mean to place it at the top level?".into())
                .build()
        );
    }

    /// [Reports](Parser::report): `token` represents an invalid start to a
    /// statement.
    fn report_invalid_token(&mut self, token: Handle<Token>) {
        self.report(
            ErrorBuilder::new()
                .of_style(Style::Primary)
                .at_level(Level::Error)
                .with_code(ErrorCode::InvalidTokenForStatement)
                .span(token)
                .message("Invalid token at the start of a statement".into())
                .build()
        );
    }

    /// [Reports](Parser::report): `token` was used incorrectly as a `usage`
    /// operator when it is not.
    fn report_invalid_operator(&mut self, token: Handle<Token>, usage: &str) {
        self.report(
            ErrorBuilder::new()
                .of_style(Style::Primary)
                .at_level(Level::Error)
                .with_code(ErrorCode::InvalidOperatorSyntax)
                .span(token)
                .message(format!(
                    "{} is not an {} operator",
                    token.value, usage
                ))
                .explain(format!("Used here as an {} operator", usage))
                .build()
        );
    }

    fn report(&mut self, error: Error) {
        self.error_manager.record(error);
    }
}

// TODO: see if you can remove or actually make useful
macro_rules! expect_n {
    ($self:ident in [$($token_type:expr),*] => $context:expr) => {
        if $self.is_eof() {
            $self.report_unexpected_eof($context.into());
            None
        } else if ![$($token_type),*].contains(&$self.current().ty) {
            $self.report_expected_tokens(
                &[$($token_type),*],
                $self.current(),
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
/// surrounded by tokens of type `left_type` and `right_type`. `name` is a
/// string that is used in error messages to describe the syntactic construct
/// for when the containing tokens are invalid.
///
/// If the `...` body returns `value`, then `contained_in!` returns
/// `Some((left_token, value, right_token))`; otherwise, it returns `None`. The
/// `...` body may use `?` try syntax. Here, `left_token` corresponds to
/// `left_type` and likewise for `right_token`.
macro_rules! contained_in {
    ($self:ident; $open_type:expr, $loc_ctx:expr, $close_type:expr; $($action:tt)*) => {
        {
            let open_token = $self.expect($open_type, ParseErrorContext::begin($loc_ctx))?;
            let result = {$($action)*};
            let close_token = $self.expect($close_type, ParseErrorContext::end($loc_ctx));
            if close_token.is_none() {
                $self.report_refback(
                    open_token,
                    format!("{} opened here", $open_type)
                );
                return None;
            }
            Some((open_token, result, close_token.unwrap()))
        }
    };
}

/// Constructs a function returning `Option<N>` for some [`NodeInterface`] `N`
/// given a function returning `Option<N::V>`, assuming that the parser's pool
/// is [`AsNodePool<N>`]. For example. `parse_full_node!(self.foo())`.
macro_rules! parse_full_node {
    ($self:ident.$method:ident($($arg:expr),*)) => {{
        let start_token = $self.current().clone();
        let value = $self.$method($($arg),*);
        value.map(|v| {
            let end_token = $self.previous().clone();
            $self.ast_pool.new(v, start_token, end_token)
        })
    }};
}

impl TokenType {
    fn begins_top_level_construct(&self) -> bool {
        matches!(self, Self::Func) // || Self::Import
    }
}

impl<'ast, 'err, P: AsASTPool> Parser<'ast, 'err, P> {
    /// Advances until EOF, or when specified by `current_exit`, or when a
    /// top-level construct is potentially found.
    fn synchronize(
        &mut self, custom_exit: fn(Handle<Token>) -> bool, description: String
    ) {
        if !self.is_eof() {
            self.report(
                ErrorBuilder::new()
                    .of_style(Style::Primary)
                    .at_level(Level::Info)
                    .with_code(ErrorCode::UnexpectedToken)
                    .span(self.current())
                    .message(
                        "Attempting to recover understanding of code".into()
                    )
                    .fix(description)
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

    /// Identical to [`Parser::synchronize`] but with no custom exit.
    fn attempt_restore_to_top_level(&mut self) {
        self.synchronize(|_| false, "Seeking top-level construct".into());
    }

    fn parse_primary_type<S: AsRef<str>>(
        &mut self, name: Option<S>
    ) -> Option<TypeValue> {
        let type_token = self.expect(
            TokenType::Identifier,
            name.map_or(
                ParseErrorContext::new().for_("primary type"),
                ParseErrorContext::new(),
                in_
            )
        )?;
        Some(match type_token.value.as_str() {
            "Int64" | "Int" => TypeValue::Int64,
            "Unit" => TypeValue::Unit,
            other => TypeValue::Name(other.into())
        })
    }

    fn parse_array_type(&mut self, inner: Handle<Type>) -> Option<TypeValue> {
        let (_, size_token, close_token) = contained_in! { self;
            TokenType::LeftBracket, "array type", TokenType::RightBracket;
            self.expect(TokenType::Integer, ParseErrorContext::For("array size".into()))?
        }?;

        let size = size_token
            .value
            .as_str()
            .parse::<i64>()
            .expect("number token can be parsed as number");
        match size.cmp(&0) {
            cmp::Ordering::Less => {
                self.report(
                    ErrorBuilder::new()
                        .of_style(Style::Primary)
                        .at_level(Level::Error)
                        .with_code(ErrorCode::MalformedType)
                        .span(size_token)
                        .message("Array size cannot be negative".into())
                        .build()
                );
                return None;
            }
            cmp::Ordering::Equal => {
                self.report(
                    ErrorBuilder::new()
                        .of_style(Style::Primary)
                        .at_level(Level::Warning)
                        .with_code(ErrorCode::MalformedType)
                        .span(size_token)
                        .message("Array size is zero".into())
                        .build()
                );
            }
            _ => {}
        }

        let result_value = TypeValue::Array(
            inner,
            self.ast_pool.generate(
                LiquidTypeValue::Equal(size as usize),
                size_token,
                size_token
            )
        );
        if self.is_at(TokenType::LeftBracket) {
            let result = self.ast_pool.new(
                result_value,
                inner.start_token(),
                close_token
            );
            self.parse_array_type(result)
        } else {
            Some(result_value)
        }
    }

    fn parse_type(&mut self, name: Option<&str>) -> Option<TypeValue> {
        if self.is_eof() {
            self.report_unexpected_eof(ParseErrorContext::in_::<&str>(
                name.unwrap_or("type")
            ));
            return None;
        }
        let primary = parse_full_node!(self.parse_primary_type(name))?;
        if self.is_at(TokenType::LeftBracket) {
            self.parse_array_type(primary)
        } else {
            Some(primary.value.clone())
        }
    }

    // ============================== EXPRESSIONS ==============================

    fn parse_array_literal_expr_value(&mut self) -> Option<ExprValue> {
        let mut elements = vec![];
        let mut should_continue = None;
        let mut i = 0;
        contained_in! { self;
            TokenType::LeftBracket, "array literal", TokenType::RightBracket;

            while !self.is_eof() && self.current().ty != TokenType::RightBracket {
                if i > 0 {
                    self.expect(
                        TokenType::Comma,
                        ParseErrorContext::Between("array elements".into())
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
                        should_continue = Some(self.current());
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
            [TokenType::Integer, TokenType::Float, TokenType::Char, TokenType::LeftBracket, TokenType::Identifier] => ParseErrorContext::Begin("literal expression".into())
        }?;
        match literal_token.ty {
            TokenType::Integer => Some(ExprValue::ConstantInt(
                literal_token.value.parse::<i64>().unwrap()
            )),
            TokenType::LeftBracket => {
                self.unget();
                self.parse_array_literal_expr_value()
            }
            TokenType::Identifier => Some(ExprValue::BoundName(literal_token)),
            _ => None
        }
    }

    fn parse_unary_prefix_expr_value(
        &mut self, prefix_op: Op
    ) -> Option<ExprValue> {
        if prefix_op.is_unary_prefix() {
            self.report_invalid_operator(self.current(), "unary");
            return None;
        }

        let op_token = self.take();
        let rhs = self.parse_expr()?;

        Some(ExprValue::PrefixOp(op_token, rhs))
    }

    fn parse_postfix_binary_expr_value(
        &mut self, mut lhs: Handle<Expr>
    ) -> Option<ExprValue> {
        while let Some(postfix_bop) =
            self.current_op_opt().and_then(|op| op.postfix_binary)
        {
            let op1 = self.take();
            let rhs = self.parse_expr()?;
            let op2 = self.expect(
                postfix_bop.close_token_ty,
                postfix_bop.name.map_or(
                    ParseErrorContext::After(
                        "second subexpression in postfix binary expression"
                            .into()
                    ),
                    ParseErrorContext::In
                )
            )?;
            let start_token = lhs.start_token();
            let end_token = op2;
            lhs = self.ast_pool.new(
                ExprValue::PostfixBop(lhs, op1, rhs, op2),
                start_token,
                end_token
            );
        }
        Some(lhs.value.clone())
    }

    fn parse_call_expr_value(&mut self) -> Option<ExprValue> {
        let name = self.expect(
            TokenType::Identifier,
            ParseErrorContext::Begin("call expression".into())
        )?;

        let mut args = vec![];
        contained_in! { self;
            TokenType::LeftPar, "call expression", TokenType::RightPar;
            let mut i = 0;
            while !self.is_eof() && self.current().ty != TokenType::RightPar {
                if i > 0 {
                    self.expect(TokenType::Comma, ParseErrorContext::Between("arguments".into()))?;
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

        Some(ExprValue::Call(name, args))
    }

    fn parse_primary_expr_value_aux(&mut self) -> Option<ExprValue> {
        if self.is_eof() {
            self.report_unexpected_eof(ParseErrorContext::Begin(
                "primary expression".into()
            ));
            None
        } else if let Some(prefix_op) =
            self.current_op_opt().filter(|op| op.is_unary_prefix())
        {
            self.parse_unary_prefix_expr_value(prefix_op)
        } else if self.is_at(TokenType::LeftPar) {
            let open_paren = self.take();
            let temp_wrapped = self.parse_expr()?;
            let expr_value = temp_wrapped.value.clone();
            let closing_paren = self.expect(
                TokenType::RightPar,
                ParseErrorContext::In("expression".into())
            );
            if closing_paren.is_none() {
                self.report_refback(
                    open_paren,
                    "Parentheses opened here".into()
                );
                None
            } else {
                Some(expr_value)
            }
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
        let primary = parse_full_node!(self.parse_primary_expr_value_aux())?;
        if self
            .current_op_opt()
            .map_or(false, |op| op.is_postfix_binary())
        {
            self.parse_postfix_binary_expr_value(primary)
        } else {
            Some(primary.value.clone())
        }
    }

    /// Implements [operator-precedence parsing](https://en.wikipedia.org/wiki/Operator-precedence_parser).
    fn parse_infix_binary_expr(
        &mut self, mut lhs: Handle<Expr>, min_precedence: Precedence
    ) -> Option<Handle<Expr>> {
        let mut lookahead = self.current();
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

            let mut rhs = parse_full_node!(self.parse_primary_expr_value())?;
            if self.is_eof() {
                break;
            }
            lookahead = self.current();
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
                lookahead = self.current();
            }
            let start_token = lhs.start_token();
            let end_token = rhs.end_token();
            lhs = self.ast_pool.new(
                ExprValue::InfixBop(lhs, op_token, rhs),
                start_token,
                end_token
            );
        }
        Some(lhs)
    }

    fn parse_expr(&mut self) -> Option<Handle<Expr>> {
        self.consume_ignored();
        let primary = parse_full_node!(self.parse_primary_expr_value())?;
        if let Some(op) = self.current_op_opt() {
            if op.is_infix_binary() {
                self.parse_infix_binary_expr(primary, -1)
            } else {
                self.report_invalid_operator(self.current(), "binary");
                None
            }
        } else {
            Some(primary)
        }
    }

    // ============================== STATEMENTS ===============================

    fn parse_let(&mut self) -> Option<StmtValue> {
        self.expect(TokenType::Let, ParseErrorContext::begin("let binding"))?;

        let name = self.expect(
            TokenType::Identifier,
            ParseErrorContext::for_("name in let binding")
        )?;

        let mut hint = None;
        if self.current().ty == TokenType::Colon {
            self.advance();
            hint = Some(parse_full_node!(
                self.parse_type(Some("let binding type hint"))
            )?);
        }

        self.expect(
            TokenType::Assign,
            ParseErrorContext::After("name in let binding".into())
        )?;

        let value = self.parse_expr()?;

        Some(StmtValue::Let { name, hint, value })
    }

    /// Requires: `lhs` has just been parsed as a valid expression in the token
    /// stream.
    fn parse_assign(&mut self, lhs: Handle<Expr>) -> Option<StmtValue> {
        let assign = self.expect(
            TokenType::Assign,
            ParseErrorContext::between("terms in assignment")
        )?;
        let rhs = self.parse_expr()?;
        Some(StmtValue::Assign(lhs, assign, rhs))
    }

    /// Parses a brace-enclosed list of statements, e.g., `parse_block("function
    /// body")`.
    fn parse_block(&mut self, name: &str) -> Option<Block> {
        contained_in! { self;
            TokenType::LeftBrace, name, TokenType::RightBrace;

            let mut nodes = vec![];

            self.consume_ignored();
            while !self.is_eof() && self.current().ty != TokenType::RightBrace {
                let stmt_opt = self.parse_stmt();
                if let Some(stmt) = stmt_opt {
                    nodes.push(stmt);
                } else {
                    self.synchronize(|token| token.ty == TokenType::RightBrace, format!("Seeking end of {}", name));
                    return None;
                }
            }

            nodes
        }
    }

    /// Requires: a statement must have been parsed, and as consequence at least
    /// one token has already been encountered.
    fn end_stmt(&mut self) -> Option<Handle<Token>> {
        let ending_token = self.previous();

        if !self.is_eof() && self.current().ty == TokenType::RightBrace {
            return Some(ending_token);
        }

        self.expect(TokenType::Newline, ParseErrorContext::after("statement"))?;

        self.consume_ignored();

        Some(ending_token)
    }

    /// Requires: `!self.is_eof()`.
    fn parse_stmt_value(&mut self) -> Option<StmtValue> {
        self.consume_ignored();
        if self.is_eof() || self.error_manager.is_full() {
            return None;
        }

        match self.current().ty {
            TokenType::Let => self.parse_let(),
            TokenType::Divider => Some(StmtValue::Divider(self.take())),
            other => {
                if let Some(expr) = self.parse_expr() {
                    return self.parse_assign(expr);
                } else if other.begins_top_level_construct() {
                    self.report_unexpected_top_level(self.current());
                } else {
                    self.report_invalid_token(self.current());
                }
                self.advance(); // to prevent infinite loop
                None
            }
        }
    }

    fn parse_stmt(&mut self) -> Option<Handle<Stmt>> {
        if self.is_eof() {
            return None;
        }
        let node = parse_full_node!(self.parse_stmt_value())?;
        if self.end_stmt().is_none() {
            self.advance();
            return None;
        }
        Some(node)
    }

    // ============================= DECLARATIONS ==============================

    fn parse_params<S: AsRef<str>, T: AsRef<str>>(
        &mut self, source: S, kind: T
    ) -> Option<(Handle<Token>, ParamVec, Handle<Token>)> {
        contained_in! { self;
            TokenType::LeftPar, source.as_ref(), TokenType::RightPar;
            self.consume_ignored();

            let mut i = 0;
            let mut params = ParamVec::new();
            while !self.is_eof() && self.current().ty != TokenType::RightPar {
                if i > 0 {
                    self.expect(
                        TokenType::Comma,
                        ParseErrorContext::between(format!(
                            "{} parameters in `{}`",
                            kind.as_ref(),
                            source.as_ref()
                        ))
                    )?;
                    self.consume_ignored();
                }
                if self.is_at(TokenType::RightPar) {
                    break;
                }

                let name = self.expect(
                    TokenType::Identifier,
                    ParseErrorContext::For(format!("{} parameter name in `{}`", kind.as_ref(), source.as_ref()))
                )?;
                self.expect(
                    TokenType::Colon,
                    ParseErrorContext::After(format!("{} parameter name in `{}`", kind.as_ref(), source.as_ref()))
                )?;
                let ty =
                    parse_full_node!(self.parse_type(Some(&format!("{} parameter type", kind.as_ref()))))?;
                params.push((name, ty));

                self.consume_ignored();
                i += 1
            }

            params
        }
    }

    fn parse_func(&mut self) -> Option<DeclValue> {
        let func = self.expect(
            TokenType::Func,
            ParseErrorContext::Begin("function declaration".into())
        )?;

        let name = self.expect(
            TokenType::Identifier,
            ParseErrorContext::For("function name".into())
        )?;

        let (_, inputs, _) = self.parse_params(&name.value, "input")?;

        let outputs = if self.is_at(TokenType::Arrow) {
            self.advance();
            let (_, outputs, _) = self.parse_params(&name.value, "output")?;
            outputs
        } else {
            ParamVec::new()
        };

        let (_open_brace, body, _close_brace) =
            self.parse_block("function body")?;

        Some(DeclValue::Function {
            func,
            name,
            inputs,
            outputs,
            body
        })
    }

    /// Requires: `!self.is_eof()`.
    fn parse_decl_value(&mut self) -> Option<DeclValue> {
        self.consume_ignored();
        if self.is_eof() || self.error_manager.is_full() {
            return None;
        }

        match self.current().ty {
            TokenType::Func => self.parse_func(),
            _ => {
                self.report_invalid_top_level(self.current());
                None
            }
        }
    }

    fn parse_decl(&mut self) -> Option<Handle<Decl>> {
        if self.is_eof() {
            return None;
        }
        if let Some(decl) = parse_full_node!(self.parse_decl_value()) {
            Some(decl)
        } else {
            self.attempt_restore_to_top_level();
            self.parse_decl()
        }
    }

    pub fn parse(mut self) -> Option<AST> {
        while self.parse_decl().is_some() {}
        if self.error_manager.has_errors() {
            None
        } else {
            Some(self.ast_pool.as_pool_mut().as_array())
        }
    }
}

impl<'ast, 'err, P: AsASTPool> Iterator for Parser<'ast, 'err, P> {
    type Item = Handle<Decl>;

    fn next(&mut self) -> Option<Handle<Decl>> {
        self.parse_decl()
    }
}
