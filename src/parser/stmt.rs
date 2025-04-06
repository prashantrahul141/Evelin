use anyhow::anyhow;
use log::trace;

use crate::{
    ast::{BlockStmt, IfStmt, LetStmt, PrintStmt, ReturnStmt, Stmt},
    token::TokenType,
};

use super::{ParserResult, parser::Parser};

impl<'a> Parser<'a> {
    pub(super) fn stmt(&mut self) -> ParserResult<Stmt> {
        if self.match_token(&[TokenType::LeftBrace]) {
            return self.block();
        } else if self.match_token(&[TokenType::Let]) {
            return self.let_decl();
        } else if self.match_token(&[TokenType::Print]) {
            return self.print_stmt();
        } else if self.match_token(&[TokenType::Return]) {
            return self.return_stmt();
        } else if self.match_token(&[TokenType::If]) {
            return self.if_stmt();
        }

        self.expression_stmt()
    }

    pub(super) fn block(&mut self) -> ParserResult<Stmt> {
        trace!("parsing block stmts.");
        let mut block_stmts = vec![];
        while !self.match_token(&[TokenType::RightBrace]) && !self.is_at_end() {
            block_stmts.push(self.stmt()?)
        }

        Ok(Stmt::Block(BlockStmt { stmts: block_stmts }))
    }

    fn let_decl(&mut self) -> ParserResult<Stmt> {
        trace!("Parsing let declaration statement");
        let name = match self.consume(
            TokenType::Identifier,
            "Expected identifier name after 'let'",
        ) {
            Some(n) => n.lexeme.clone(),
            None => return Err(anyhow!("Expected identifier name after 'let'")),
        };

        self.consume(TokenType::Equal, "Expected '=' after identifier name");
        let initialiser = self.expr()?;
        self.consume(TokenType::Semicolon, "Expected ';' after let statement");

        let stmt = Stmt::Let(LetStmt { name, initialiser });
        Ok(stmt)
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
            return Ok(Stmt::Print(PrintStmt { value: expr }));
        }

        Err(anyhow!("Failed to parse print statement."))
    }

    fn return_stmt(&mut self) -> ParserResult<Stmt> {
        trace!("Parsing return stmt");
        let return_value = self.expr();
        self.consume(TokenType::Semicolon, "Expected ';' after return statement");

        if let Ok(expr) = return_value {
            return Ok(Stmt::Return(ReturnStmt { value: expr }));
        }

        Err(anyhow!("Failed to parse return statement."))
    }

    fn expression_stmt(&mut self) -> ParserResult<Stmt> {
        trace!("Parsing expression stmt");
        let expr = self.expr()?;
        self.consume(TokenType::Semicolon, "Expected ';' after expression");
        Ok(Stmt::Expression(expr))
    }
}
