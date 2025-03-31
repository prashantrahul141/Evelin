use anyhow::anyhow;
use log::{debug, error, trace};

use crate::{
    ast::{BinExpr, BinOp, Expr, GroupExpr, LiteralExpr, UnOp, UnaryExpr},
    die,
    token::{LiteralValue, Token, TokenType},
};

pub struct Parser<'a> {
    /// Vec of tokens to parse.
    tokens: &'a Vec<Token>,

    /// Current token position.
    current: usize,

    /// flag to be set if any error occurs during parsing.
    has_errors: bool,

    /// vec of parsed ast.
    pub stmts: Vec<Expr>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        debug!("Start parsing--------------------------------");
        Self {
            tokens,
            current: 0,
            has_errors: false,
            stmts: vec![],
        }
    }

    /// Public api to start parsing.
    pub fn parse(&mut self) {
        while !self.is_at_end() {
            trace!("parsing new stmt.");
            match self.expr() {
                Ok(v) => self.stmts.push(v),
                Err(_) => todo!("me when parsing error"),
            }
        }
    }

    /// Parses top-level expressions.
    fn expr(&mut self) -> Result<Expr, anyhow::Error> {
        self.term()
    }

    /// Parses term expressions.
    fn term(&mut self) -> Result<Expr, anyhow::Error> {
        let mut left = self.factor()?;

        while self.match_token(&[TokenType::Minus, TokenType::Plus]) {
            let op = BinOp::from(&self.previous().ttype);
            let right = self.factor()?;
            let bin = BinExpr {
                left: left.clone(),
                op,
                right,
            };
            left = Expr::Binary(Box::new(bin));
        }

        Ok(left)
    }

    /// Parses factor expressions.
    fn factor(&mut self) -> Result<Expr, anyhow::Error> {
        let mut left = self.unary()?;

        while self.match_token(&[TokenType::Slash, TokenType::Star, TokenType::Mod]) {
            let op = BinOp::from(&self.previous().ttype);
            let right = self.unary()?;
            let bin = BinExpr {
                left: left.clone(),
                op,
                right,
            };
            left = Expr::Binary(Box::new(bin));
        }

        Ok(left)
    }

    /// Parses unary expressions.
    fn unary(&mut self) -> Result<Expr, anyhow::Error> {
        if self.match_token(&[TokenType::Bang, TokenType::Minus]) {
            let op = UnOp::from(&self.previous().ttype);
            if let Ok(operand) = self.unary() {
                let un = UnaryExpr { op, operand };
                return Ok(Expr::Unary(Box::new(un)));
            }
        }

        let primary = self.primary();
        trace!("Parser::primary = {:?}", primary);
        primary
    }

    /// Parses literal expressions.
    fn primary(&mut self) -> Result<Expr, anyhow::Error> {
        if self.match_token(&[TokenType::False]) {
            let literal = Expr::Literal(LiteralExpr {
                value: LiteralValue::Boolean(false),
            });

            return Ok(literal);
        }

        if self.match_token(&[TokenType::True]) {
            let literal = Expr::Literal(LiteralExpr {
                value: LiteralValue::Boolean(true),
            });

            return Ok(literal);
        }

        // literal values get parsed in the lexer section, we can just clone it here.
        if self.match_token(&[
            TokenType::Null,
            TokenType::String,
            TokenType::NumberInt,
            TokenType::NumberFloat,
        ]) {
            let literal = Expr::Literal(LiteralExpr {
                value: self.previous().literal.clone(),
            });

            return Ok(literal);
        }

        // grouping
        if self.match_token(&[TokenType::LeftParen]) {
            trace!("Grouping found.");
            if let Ok(expr) = self.expr() {
                self.consume(
                    TokenType::RightParen,
                    format!("Expected ')' got {} instead", self.current().ttype).as_str(),
                );
                let literal = Expr::Grouping(Box::new(GroupExpr { value: expr }));
                return Ok(literal);
            }
        }

        let error = self.report_parser_error(format!(
            "Expected literal value recieved {} instead.",
            self.current().literal
        ));

        Err(anyhow!(error))
    }

    /// Consumes current token if it matches the given token type.
    /// * `expected_type` - type of token to match with.
    /// * `message` - error message for when token doesn't match the expected type
    fn consume(&mut self, expected_type: TokenType, message: &str) -> Option<&Token> {
        if self.match_current(&expected_type) {
            return Some(self.advance());
        }

        self.report_parser_error(message);
        die!("{}", message);
    }

    /// Checks whether the current token is
    /// present in the given expected vec of tokens
    /// * `expected` - expected vec of tokens.
    fn match_token(&mut self, expected: &[TokenType]) -> bool {
        for ttype in expected {
            if self.match_current(ttype) {
                self.advance();
                return true;
            }
        }

        false
    }

    /// Checks whether the current token is of given type
    /// * `expected_type` - the type to match with.
    fn match_current(&self, expected_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        expected_type == &self.current().ttype
    }

    /// Consumes current token and returns it.
    /// if reached the end, keeps returning the last token.
    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        let prev = self.previous();
        trace!("Parser::advance self.previous() = {}", &prev);
        prev
    }

    /// Returns current token.
    fn current(&self) -> &Token {
        &self.tokens[self.current]
    }

    /// Returns previous token.
    fn previous(&self) -> &Token {
        if self.current == 0 {
            die!("Parser::previous self.current = 0");
        }
        &self.tokens[self.current - 1]
    }

    /// Checks whether the end of the token stream has been reached.
    fn is_at_end(&self) -> bool {
        self.current().ttype == TokenType::Eof
    }

    /// Sets parser's error flag, then logs the given error message.
    /// * `message` - error message.
    fn report_parser_error<T: Into<String>>(&mut self, message: T) -> String {
        self.has_errors = true;
        let value: String = message.into();
        error!(
            "Parsing error: at line {}: {:?}",
            self.previous().line,
            value
        );

        value
    }
}
