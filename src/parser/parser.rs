use log::{debug, trace};

use crate::{ast::Expr, token::Token};

pub struct Parser<'a> {
    /// Vec of tokens to parse.
    pub(super) tokens: &'a Vec<Token>,

    /// Current token position.
    pub(super) current: usize,

    /// flag to be set if any error occurs during parsing.
    pub has_errors: bool,

    /// vec of parsed ast.
    pub stmts: Vec<Expr>,
}

impl<'a> From<&'a Vec<Token>> for Parser<'a> {
    fn from(tokens: &'a Vec<Token>) -> Self {
        debug!("Start parsing--------------------------------");
        Self {
            tokens,
            current: 0,
            has_errors: false,
            stmts: vec![],
        }
    }
}

impl<'a> Parser<'a> {
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
}
