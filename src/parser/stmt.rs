use anyhow::anyhow;
use log::trace;

use crate::{
    ast::{IfStmt, PrintStmt, ReturnStmt, Stmt},
    token::TokenType,
};

use super::{ParserResult, parser::Parser};

impl<'a> Parser<'a> {
    // fn if_stmt(&mut self) -> ParserResult<Stmt> {
    //     self.consume(TokenType::LeftParen, "Expected '(' after if statement");
    //     let condition = self.expr();
    // }

    pub(super) fn stmt(&mut self) -> ParserResult<Stmt> {
        if self.match_token(&[TokenType::Print]) {
            return self.print_stmt();
        } else if self.match_token(&[TokenType::Return]) {
            return self.return_stmt();
        } else if self.match_token(&[TokenType::If]) {
            return self.if_stmt();
        }

        Err(anyhow!("Invalid statement type."))
    }

    fn if_stmt(&mut self) -> ParserResult<Stmt> {
        trace!("Parsing if stmt");
        self.consume(TokenType::LeftParen, "Expected '(' after if statement");
        let condition = self.expr()?;
        self.consume(TokenType::RightParen, "Expected ')' after if statement");

        let if_branch = self.stmt()?;

        let mut else_branch = None;

        if self.match_token(&[TokenType::Else]) {
            trace!("found else branch in if stmt, parsing.");
            else_branch = Some(self.stmt()?);
        }

        Ok(Stmt::If(Box::new(IfStmt {
            condition,
            if_branch,
            else_branch,
        })))
    }

    fn print_stmt(&mut self) -> ParserResult<Stmt> {
        trace!("Parsing print stmt");
        let value = self.expr();
        self.consume(TokenType::Semicolon, "Expected ';' after print statement");

        if let Ok(expr) = value {
            return Ok(Stmt::Print(PrintStmt { expr }));
        }

        Err(anyhow!("Failed to parse print statement."))
    }

    fn return_stmt(&mut self) -> ParserResult<Stmt> {
        trace!("Parsing return stmt");
        let return_value = self.expr();
        self.consume(TokenType::Semicolon, "Expected ';' after return statement");

        if let Ok(expr) = return_value {
            return Ok(Stmt::Return(ReturnStmt { expr }));
        }

        Err(anyhow!("Failed to parse return statement."))
    }
}
