use super::{
    ast::Node,
    token::{Token, TokenType}
};
use crate::{
    frontend::ast::NodeValue,
    utils::error::{
        Error, ErrorBuilder, ErrorCode, ErrorManager, Level, Style
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

    fn peek(&self) -> Option<&Token> {
        if self.pos + 1 < self.buffer.len() {
            Some(&self.buffer[self.pos + 1])
        } else {
            None
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

    fn consume_ignored(&mut self) {
        while !self.is_eof() && self.current().ty == TokenType::Newline {
            self.advance()
        }
    }

    /// Error for when EOF is encountered in the parsing context `context`.
    ///
    /// Requires: `!buffer.is_empty()`.
    fn error_unexpected_eof(&self, context: &str) -> Error {
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
}

macro_rules! expect {
    ($self:ident in $token_type:expr => $context:expr) => {
        if $self.is_eof() {
            $self
                .error_manager
                .borrow_mut()
                .record($self.error_unexpected_eof($context));
            None
        } else if $self.current().ty != $token_type {
            $self.error_manager.borrow_mut().record(
                $self.error_expected_token(
                    $token_type,
                    $self.current(),
                    $context
                )
            );
            None
        } else {
            Some($self.take())
        }
    };
}

impl TokenType {
    fn begins_top_level_construct(&self) -> bool {
        match self {
            Self::Func => true,
            _ => false
        }
    }
}

impl Parser {
    /// Advances until EOF or a top-level construct is potentially found.

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

    fn end_stmt(&mut self) -> Option<()> {
        if !self.is_eof() && self.current().ty == TokenType::RightBrace {
            return Some(());
        }

        expect! { self in TokenType::Newline => "after statement" }?;

        self.consume_ignored();

        Some(())
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
            self.error_manager.borrow_mut().record(self.error_refback(
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
        expect! { self in
            TokenType::Func => "at start of function declaration"
        }?;

        let name =
            expect! { self in TokenType::Identifier => "for function name" }?;

        let open_paren = expect! { self in TokenType::LeftPar => "at start of function parameters" }?;
        // TODO: params
        let close_paren = expect! { self in TokenType::RightPar => "at end of function parameters" };
        if close_paren.is_none() {
            self.error_manager.borrow_mut().record(self.error_refback(
                &open_paren,
                "Opening parenthesis opened here".into()
            ));
            return None;
        }

        let body = self.parse_block("function body")?;

        Some(Node {
            value: NodeValue::Function {
                name: name.clone(),
                body
            },
            ty: None
        })
    }

    /// Do not call this function directly.
    fn parse_stmt_aux(&mut self, top_level: bool) -> Option<Node> {
        self.consume_ignored();
        if self.is_eof() || self.error_manager.borrow().is_full() {
            return None;
        }

        let current_ty = self.current().ty;
        match (current_ty, top_level) {
            (TokenType::Func, true) => {
                if let Some(func) = self.parse_func() {
                    return Some(func);
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
                self.error_manager.borrow_mut().record(stmt_error);
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
        self.parse_stmt_aux(top_level).and_then(|node| {
            self.end_stmt()?;
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
