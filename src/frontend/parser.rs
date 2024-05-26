use super::{
    ast::{Expr, ExprValue, Node},
    op::{Op, Precedence},
    token::{Token, TokenType},
    ty::{StmtType, Type, TypeCell}
};
use crate::{
    frontend::ast::NodeValue,
    utils::{
        error::{Error, ErrorBuilder, ErrorCode, ErrorManager, Level, Style},
        mutcell::MutCell
    }
};
use std::{cell::RefCell, rc::Rc};

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
        self.pos += 1
    }

    fn take(&mut self) -> Token {
        let result = self.current().clone();
        self.advance();
        result
    }

    fn unget(&mut self) -> bool {
        if self.pos > 0 {
            self.pos -= 1;
            true
        } else {
            false
        }
    }

    fn consume_ignored(&mut self) {
        while !self.is_eof() && self.current().ty == TokenType::Newline {
            self.advance()
        }
    }

    /// Error for when EOF is encountered in the parsing context `context`.
    ///
    /// Requires: `!buffer.is_empty()`.
    fn error_unexpected_eof(&self, context: String) -> Error {
        ErrorBuilder::new()
            .of_style(Style::Primary)
            .at_level(Level::Error)
            .with_code(ErrorCode::UnexpectedEOF)
            .at_token(self.buffer.last().unwrap())
            .explain(format!("Unexpected EOF {}", context))
            .build()
    }

    /// Error for when the found type of the token `actual` diverges from the
    /// expected type `expected_ty` in the parsing context `context`.
    fn error_expected_token(
        &self, expected_ty: TokenType, actual: &Token, context: &str
    ) -> Error {
        ErrorBuilder::new()
            .of_style(Style::Primary)
            .at_level(Level::Error)
            .with_code(ErrorCode::UnexpectedToken)
            .at_token(actual)
            .message(format!("Expected '{:?}' {}", expected_ty, context))
            .explain(format!("Received '{:?}' here", actual.ty))
            .build()
    }

    /// @see [`Parser::error_expected_token`]
    fn error_expected_tokens(
        &self, expected_tys: &[TokenType], actual: &Token, context: &str
    ) -> Error {
        ErrorBuilder::new()
            .of_style(Style::Primary)
            .at_level(Level::Error)
            .with_code(ErrorCode::UnexpectedToken)
            .at_token(actual)
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
    }

    /// Error for when referring back to a previous token `refback` with
    /// additional explanation `explain`.
    fn error_refback(&self, refback: &Token, explain: String) -> Error {
        ErrorBuilder::new()
            .of_style(Style::Secondary)
            .at_level(Level::Error)
            .at_token(refback)
            .message("   ...".into())
            .explain(explain)
            .build()
    }

    /// Error for when a construct (marked by `token`) is found at top level
    /// that should not be.
    fn error_top_level(&self, token: &Token) -> Error {
        ErrorBuilder::new()
            .of_style(Style::Primary)
            .at_level(Level::Error)
            .with_code(ErrorCode::InvalidTopLevelConstruct)
            .at_token(token)
            .message(format!("Unexpected {:?} at top level", token.ty))
            .fix(
                "Allowed constructs at top level include functions and imports"
                    .into()
            )
            .build()
    }

    /// Error for when a construct (marked by `token`) that belongs only at top
    /// level is found further nested.
    fn error_not_top_level(&self, token: &Token) -> Error {
        ErrorBuilder::new()
            .of_style(Style::Primary)
            .at_level(Level::Error)
            .with_code(ErrorCode::ConstructShouldBeTopLevel)
            .at_token(token)
            .message("Unexpected top-level construct".into())
            .fix("Did you mean to place it at the top level?".into())
            .build()
    }

    /// Error for when `token` represents an invalid start to a statement.
    fn error_invalid_token(&self, token: &Token) -> Error {
        ErrorBuilder::new()
            .of_style(Style::Primary)
            .at_level(Level::Error)
            .with_code(ErrorCode::InvalidTokenForStatement)
            .at_token(token)
            .message("Invalid token at the start of statement".into())
            .build()
    }

    fn error_invalid_operator(&self, token: &Token, usage: &str) -> Error {
        ErrorBuilder::new()
            .of_style(Style::Primary)
            .at_level(Level::Error)
            .with_code(ErrorCode::InvalidOperatorSyntax)
            .at_token(token)
            .message(format!("{} is not an {} operator", token.value, usage))
            .explain(format!("Used here as an {} operator", usage))
            .build()
    }

    fn report(&mut self, error: Error) {
        self.error_manager.borrow_mut().record(error);
    }
}

// macro_rules! expect_any {
//     ($self:ident in _ => $context:expr) => {
//         if $self.is_eof() {
//             $self.report($self.error_unexpected_eof($context));
//             None
//         } else {
//             Some($self.take())
//         }
//     };
// }

macro_rules! expect {
    ($self:ident in $token_type:expr => $context:expr) => {
        if $self.is_eof() {
            $self.report($self.error_unexpected_eof($context.into()));
            None
        } else if $self.current().ty != $token_type {
            $self.report($self.error_expected_token(
                $token_type,
                $self.current(),
                $context
            ));
            None
        } else {
            Some($self.take())
        }
    };
}

macro_rules! expect_n {
    ($self:ident in [$($token_type:expr),*] => $context:expr) => {
        if $self.is_eof() {
            $self.report($self.error_unexpected_eof($context.into()));
            None
        } else if ![$($token_type),*].contains(&$self.current().ty) {
            $self.report($self.error_expected_tokens(
                &[$($token_type),*],
                $self.current(),
                $context
            ));
            None
        } else {
            Some($self.take())
        }
    };
}

impl TokenType {
    fn begins_top_level_construct(&self) -> bool {
        match self {
            Self::Func | Self::Pure => true,
            _ => false
        }
    }
}

impl Parser {
    /// Advances until EOF, or when specified by `current_exit`, or when a
    /// top-level construct is potentially found.
    fn synchronize(&mut self, custom_exit: fn(&Token) -> bool) {
        while !self.is_eof()
            && !custom_exit(self.current())
            && !self.current().ty.begins_top_level_construct()
        {
            self.advance();
        }
    }

    /// Identical to [`Parser::synchronize`] but with no custom exit.`
    fn attempt_restore_to_top_level(&mut self) {
        self.synchronize(|_| false);
    }

    fn parse_primary_type(&mut self, name: Option<&str>) -> Option<Type> {
        let context = match name {
            Some(name) => format!("in {}", name),
            None => "for primary type".into()
        };
        let type_token = expect! { self in
            TokenType::Identifier => context.as_str()
        }?;
        Some(match type_token.value.as_str() {
            "Int64" | "Int" => Type::Int64,
            "Unit" => Type::Unit,
            other => Type::Name(other.into())
        })
    }

    fn parse_array_type(&mut self, inner: Type) -> Option<Type> {
        let open_bracket =
            expect! { self in TokenType::LeftBracket => "in array type" }?;
        let size_token =
            expect! { self in TokenType::Integer => "for array size" }?;
        let close_brace =
            expect! { self in TokenType::RightBracket => "in array type" };
        if close_brace.is_none() {
            self.report(
                self.error_refback(&open_bracket, "Bracket opened here".into())
            );
            return None;
        }

        let size = i64::from_str_radix(size_token.value.as_str(), 10).ok()?;
        if size < 0 {
            self.report(
                ErrorBuilder::new()
                    .of_style(Style::Primary)
                    .at_level(Level::Error)
                    .with_code(ErrorCode::MalformedType)
                    .at_token(&size_token)
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
                    .at_token(&size_token)
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
            self.report(self.error_unexpected_eof(match name {
                Some(name) => format!("in {}", name),
                None => "in type".into()
            }));
            return None;
        }
        let primary = self.parse_primary_type(name)?;
        if self.is_at(TokenType::LeftBracket) {
            self.parse_array_type(primary)
        } else {
            Some(primary)
        }
    }

    fn parse_array_expr(&mut self) -> Option<Expr> {
        let open_bracket = expect! { self in TokenType::LeftBracket => "to start array literal" }?;

        let mut elements = vec![];
        let mut should_continue = false;
        let mut i = 0;
        while !self.is_eof() && self.current().ty != TokenType::RightBracket {
            if i > 0 {
                expect! { self in TokenType::Comma => "between array elements" }?;
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
                self.synchronize(|token| token.ty == TokenType::RightBrace);
                return None;
            }

            i += 1;
        }

        let close_brace = expect! { self in TokenType::RightBracket => "to end array literal" };
        if close_brace.is_none() {
            self.report(
                self.error_refback(&open_bracket, "Bracket opened here".into())
            );
            return None;
        }

        Some(Expr {
            value: ExprValue::ArrayLiteral(elements, should_continue),
            ty: Type::make_unknown(),
            start: open_bracket
        })
    }

    fn parse_literal_expr(&mut self) -> Option<Expr> {
        let literal_token = expect_n! { self in
            [TokenType::Integer, TokenType::Float, TokenType::Char, TokenType::LeftBracket, TokenType::Identifier] => "at start of expression"
        }?;
        match literal_token.ty {
            TokenType::Integer => Some(Expr {
                value: ExprValue::ConstantInt(
                    i64::from_str_radix(&literal_token.value, 10).unwrap()
                ),
                ty: Type::make_unknown(),
                start: literal_token
            }),
            TokenType::LeftBracket => {
                self.unget();
                self.parse_array_expr()
            }
            TokenType::Identifier => Some(Expr {
                value: ExprValue::BoundName(literal_token.clone()),
                ty: Type::make_unknown(),
                start: literal_token
            }),
            _ => None
        }
    }

    fn parse_prefix_expr(
        &mut self, prefix_op: Op, start: Token
    ) -> Option<Expr> {
        if !prefix_op.is_unary {
            self.report(self.error_invalid_operator(self.current(), "unary"));
            return None;
        }

        let op_token = self.take();
        let rhs = self.parse_expr()?;

        Some(Expr {
            value: ExprValue::PrefixOp(op_token, Box::new(rhs)),
            ty: Type::make_unknown(),
            start
        })
    }

    fn parse_call_expr(&mut self) -> Option<Expr> {
        let name = expect! { self in TokenType::Identifier => "at start of call expression" }?;
        let open_par =
            expect! { self in TokenType::LeftPar => "in call expression" }?;

        let mut args = vec![];
        let mut i = 0;
        while !self.is_eof() && self.current().ty != TokenType::RightPar {
            if i > 0 {
                expect! { self in TokenType::Comma => "between array elements" }?;
                self.consume_ignored();
            }
            if self.is_at(TokenType::RightPar) {
                break;
            }

            let arg_opt = self.parse_expr();
            if let Some(arg) = arg_opt {
                args.push(arg);
            } else {
                self.synchronize(|token| token.ty == TokenType::RightPar);
                return None;
            }

            i += 1;
        }

        let closing_paren = expect! { self in
            TokenType::RightPar => "at end of call expression"
        };
        if closing_paren.is_none() {
            self.report(
                self.error_refback(&open_par, "Parentheses opened here".into())
            );
            None
        } else {
            Some(Expr {
                value: ExprValue::Call(name.clone(), args),
                ty: Type::make_unknown(),
                start: name
            })
        }
    }

    fn parse_primary_expr(&mut self) -> Option<Expr> {
        if self.is_eof() {
            self.report(self.error_unexpected_eof("in expression".into()));
            None
        } else if let Some(prefix_op) = Op::from(self.current().ty) {
            self.parse_prefix_expr(prefix_op, self.current().clone())
        } else if self.is_at(TokenType::LeftPar) {
            let open_paren = self.take();
            let expr = self.parse_expr()?;
            let closing_paren = expect! { self in
                TokenType::RightPar => "in expression"
            };
            if closing_paren.is_none() {
                self.report(self.error_refback(
                    &open_paren,
                    "Parentheses opened here".into()
                ));
                None
            } else {
                Some(expr)
            }
        } else if self.is_at(TokenType::HardwareMap) {
            let map_token = expect! { self in TokenType::HardwareMap => "at start of hardware map" }?;
            expect! { self in TokenType::LeftAngle => "in hardware map expression" }?;
            let parallel_factor_token = expect! { self in TokenType::Integer => "in hardware map expression" }?;
            expect! { self in TokenType::RightAngle => "in hardware map expression" }?;
            expect! { self in TokenType::LeftPar => "in hardware map expression" }?;
            let f = expect! { self in TokenType::Identifier => "in hardware map expression" }?;
            expect! { self in TokenType::Comma => "in hardware map expression" }?;
            let arr = self.parse_expr()?;
            expect! { self in TokenType::RightPar => "in hardware map expression" }?;
            // TODO: check for negatives
            Some(Expr {
                value: ExprValue::HardwareMap(
                    map_token.clone(),
                    usize::from_str_radix(&parallel_factor_token.value, 10)
                        .unwrap(),
                    f,
                    Box::new(arr)
                ),
                start: map_token,
                ty: Type::make_unknown()
            })
        } else if self.is_at(TokenType::Identifier)
            && self.next_is(TokenType::LeftPar)
        {
            // TODO: allow calling expressions and more complex names with `::`
            self.parse_call_expr()
        } else {
            self.parse_literal_expr()
        }
    }

    /// Implements [operator-precedence parsing](https://en.wikipedia.org/wiki/Operator-precedence_parser).
    fn parse_binary_expr(
        &mut self, lhs: Expr, min_precedence: Precedence, start: Token
    ) -> Option<Expr> {
        let mut lhs = lhs;
        let mut lookahead = self.current().clone();
        while !self.is_eof()
            && Op::from(lookahead.ty)
                .map(|op| op.is_binary && op.binary_precedence > min_precedence)
                .unwrap_or_default()
        {
            let op_token = self.take();
            let op = Op::from(op_token.ty).unwrap();

            let mut rhs = self.parse_primary_expr()?;
            lookahead = self.current().clone();
            while !self.is_eof()
                && Op::from(lookahead.ty)
                    .map(|next_op| {
                        next_op.is_binary
                            && ((next_op.is_left_associative
                                && next_op.binary_precedence
                                    > op.binary_precedence)
                                || (!next_op.is_left_associative
                                    && next_op.binary_precedence
                                        == op.binary_precedence))
                    })
                    .unwrap_or_default()
            {
                let next_op = Op::from(lookahead.ty).unwrap();
                rhs = self.parse_binary_expr(
                    rhs,
                    op.binary_precedence
                        + if next_op.binary_precedence > op.binary_precedence {
                            1
                        } else {
                            0
                        },
                    start.clone()
                )?;
                lookahead = self.current().clone();
            }
            lhs = Expr {
                value: ExprValue::BinOp(Box::new(lhs), op_token, Box::new(rhs)),
                ty: Type::make_unknown(),
                start: start.clone()
            };
        }
        Some(lhs)
    }

    fn parse_expr(&mut self) -> Option<Expr> {
        self.consume_ignored();
        let start = self.current().clone();
        let primary = self.parse_primary_expr()?;
        if let Some(binary_op) =
            self.current_opt().map(|token| Op::from(token.ty)).flatten()
        {
            if binary_op.is_binary {
                self.parse_binary_expr(primary, -1, start)
            } else {
                self.report(
                    self.error_invalid_operator(self.current(), "binary")
                );
                None
            }
        } else {
            Some(primary)
        }
    }

    fn parse_let(&mut self) -> Option<Node> {
        expect! { self in TokenType::Let => "at start of let binding" }?;

        let name = expect! { self in
            TokenType::Identifier => "for name in let binding"
        }?;

        let mut hint = None;
        if self.current().ty == TokenType::Colon {
            self.advance();
            hint = Some(TypeCell::new(
                self.parse_type("let binding type hint".into())?
            ));
        }

        expect! { self in TokenType::Assign => "after name in let binding" }?;

        let value = self.parse_expr()?;

        Some(Node {
            value: NodeValue::LetBinding {
                name,
                hint,
                value: Box::new(value)
            },
            ty: StmtType::make_unknown(),
            start_token: MutCell::new(None),
            end_token: MutCell::new(None)
        })
    }

    fn parse_return(&mut self) -> Option<Node> {
        let token =
            expect! { self in TokenType::Return => "return statement" }?;

        let value = if self.is_at(TokenType::Newline) {
            None
        } else {
            Some(Box::new(self.parse_expr()?))
        };

        Some(Node {
            value: NodeValue::Return { token, value },
            ty: StmtType::make_unknown(),
            start_token: MutCell::new(None),
            end_token: MutCell::new(None)
        })
    }

    /// Parses a brace-enclosed list of statements, e.g., `parse_block("function
    /// body")`.
    fn parse_block(&mut self, name: &str) -> Option<Vec<Node>> {
        self.consume_ignored();

        let opening_brace = expect! { self in
            TokenType::LeftBrace => format!("at start of {}", name).as_str()
        }?;

        self.consume_ignored();

        let mut nodes = vec![];
        let mut block_failed = false;
        while !self.is_eof() && self.current().ty != TokenType::RightBrace {
            let stmt_opt = self.parse_stmt(false);
            if let Some(stmt) = stmt_opt {
                nodes.push(stmt);
            } else {
                block_failed = true;
                self.synchronize(|token| token.ty == TokenType::RightBrace);
                break;
            }
        }

        let closing_brace = expect! { self in
            TokenType::RightBrace => format!("at end of {}", name).as_str()
        };
        if closing_brace.is_none() {
            self.report(self.error_refback(
                &opening_brace,
                format!("{} opened here", name)
            ));
            return None;
        }

        if block_failed {
            None
        } else {
            Some(nodes)
        }
    }

    fn parse_func(&mut self) -> Option<Node> {
        let mut pure_token = None;
        if self.is_at(TokenType::Pure) {
            pure_token = Some(self.take());
        }

        expect! { self in
            TokenType::Func => "at start of function declaration"
        }?;

        let name =
            expect! { self in TokenType::Identifier => "for function name" }?;

        let open_paren = expect! { self in TokenType::LeftPar => format!("at start of function parameters in `{}`", name.value).as_str() }?;

        self.consume_ignored();

        let mut i = 0;
        let mut params = vec![];
        while !self.is_eof() && self.current().ty != TokenType::RightPar {
            if i > 0 {
                expect! { self in TokenType::Comma => format!("between function parameters in `{}`", name.value).as_str() }?;
                self.consume_ignored();
            }
            if self.is_at(TokenType::RightPar) {
                break;
            }

            let name = expect! { self in TokenType::Identifier => format!("for parameter name in `{}`", name.value).as_str() }?;
            expect! { self in TokenType::Colon => format!("after parameter name in `{}`", name.value).as_str() }?;
            let ty = self.parse_type("parameter type".into())?;
            params.push((name, ty));

            self.consume_ignored();
            i += 1
        }

        let close_paren = expect! { self in TokenType::RightPar => "at end of function parameters" };
        if close_paren.is_none() {
            self.report(
                self.error_refback(
                    &open_paren,
                    "Parentheses opened here".into()
                )
            );
            return None;
        }

        let mut ret = Type::Unit;
        if self.is_at(TokenType::Arrow) {
            self.advance();
            ret = self.parse_type("function return type".into())?;
        }

        let mut body = self.parse_block("function body")?;
        if ret == Type::Unit {
            body.push(Node {
                value: NodeValue::Return {
                    token: name.clone(),
                    value: None
                },
                ty: StmtType::make_unknown(),
                start_token: MutCell::new(None),
                end_token: MutCell::new(None)
            });
        }

        Some(Node {
            value: NodeValue::Function {
                name: name.clone(),
                params,
                ret,
                pure_token,
                body
            },
            ty: StmtType::make_unknown(),
            start_token: MutCell::new(None),
            end_token: MutCell::new(None)
        })
    }

    fn end_stmt(&mut self) -> Option<Token> {
        if !self.is_eof() && self.current().ty == TokenType::RightBrace {
            return Some(self.current().clone());
        }

        let end = expect! { self in TokenType::Newline => "after statement" }?;

        self.consume_ignored();

        Some(end)
    }

    /// Do not call this function directly.
    fn parse_stmt_aux(&mut self, top_level: bool) -> Option<Node> {
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
                let stmt_error = if current_ty.begins_top_level_construct()
                    && !top_level
                {
                    self.error_not_top_level(self.current())
                } else if !current_ty.begins_top_level_construct() && top_level
                {
                    self.error_top_level(self.current())
                } else {
                    self.error_invalid_token(self.current())
                };
                self.report(stmt_error);
            }
        }

        if top_level {
            self.attempt_restore_to_top_level();
            self.parse_stmt_aux(true)
        } else {
            None
        }
    }

    fn parse_stmt(&mut self, top_level: bool) -> Option<Node> {
        let start = self.current_opt().map(|token| token.clone());
        self.parse_stmt_aux(top_level).and_then(|node| {
            let end = self.end_stmt()?;
            *node.start_token.as_mut() = Some(start.unwrap());
            *node.end_token.as_mut() = Some(end);
            Some(node)
        })
    }
}

impl Iterator for Parser {
    type Item = Node;

    fn next(&mut self) -> Option<Node> {
        self.parse_stmt(true)
    }
}
