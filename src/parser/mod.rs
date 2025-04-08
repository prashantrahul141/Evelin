use anyhow::anyhow;
use log::info;

mod expr;
mod stmt;
mod top_level;
mod utils;

pub const MAX_NATIVE_FUNCTION_ARITY: usize = 256;

pub type ParserResult<T> = anyhow::Result<T>;

use crate::ast::{FnDecl, StructDecl, Token, TokenType};

pub struct Parser<'a> {
    /// Vec of tokens to parse.
    pub(super) tokens: &'a Vec<Token>,

    /// Current token position.
    pub(super) current: usize,

    /// flag to be set if any error occurs during parsing.
    pub has_errors: bool,

    /// vec of all parsed struct declarations.
    pub struct_decls: Vec<StructDecl>,

    /// vec of all parsed function declarations.
    pub fn_decls: Vec<FnDecl>,
}

impl<'a> From<&'a Vec<Token>> for Parser<'a> {
    fn from(tokens: &'a Vec<Token>) -> Self {
        info!("Start parsing");
        Self {
            tokens,
            current: 0,
            has_errors: false,
            struct_decls: vec![],
            fn_decls: vec![],
        }
    }
}

impl Parser<'_> {
    /// Public api to start parsing.
    /// Calls (Parser::parse_internal)[parse_internal] in a loop.
    pub fn parse(&mut self) {
        while !self.is_at_end() {
            self.parse_internal();
        }
    }

    /// Internal parsing function, calls struct_decl or fn_decl as needed, reports parser error.
    fn parse_internal(&mut self) {
        if self.match_token(&[TokenType::Struct]) {
            match self.struct_decl() {
                Ok(decl) => self.struct_decls.push(decl),
                Err(e) => self.report_parser_error(e),
            };
        } else if self.match_token(&[TokenType::Fn]) {
            match self.fn_decl() {
                Ok(decl) => self.fn_decls.push(decl),
                Err(e) => self.report_parser_error(e),
            };
        } else {
            self.report_parser_error(anyhow!("Expected struct or function declaration."));
        }
    }
}
