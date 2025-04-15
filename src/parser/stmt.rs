use log::trace;

use crate::ast::{
    BlockStmt, IfStmt, LetStmt, PrintStmt, ReturnStmt, Stmt, StructInitStmt, TokenType,
};

use super::{Parser, ParserResult};

impl Parser<'_> {
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
            match self.stmt() {
                Ok(stmt) => block_stmts.push(stmt),
                Err(err) => self.report_parser_error(err),
            }
        }

        Ok(Stmt::Block(BlockStmt { stmts: block_stmts }))
    }

    fn let_decl(&mut self) -> ParserResult<Stmt> {
        trace!("Parsing let declaration statement");
        let name = self
            .consume(
                TokenType::Identifier,
                "Expected identifier name after 'let'",
            )?
            .lexeme
            .clone();

        self.consume(TokenType::Equal, "Expected '=' after identifier name")?;

        if self.match_current(&TokenType::Identifier) {
            let struct_name = self.advance().lexeme.clone();
            self.consume(TokenType::LeftBrace, "Expected '{' after struct name")?;
            let mut arguments = vec![];

            while !self.match_token(&[TokenType::RightBrace]) && !self.is_at_end() {
                let arg_name = self
                    .consume(
                        TokenType::Identifier,
                        "Expected field name inside struct initialiser",
                    )?
                    .lexeme
                    .clone();
                self.consume(
                    TokenType::Colon,
                    "Expected ':' after field name in struct initialiser",
                )?;
                let arg = self.expr()?;
                arguments.push((arg_name, arg));
                if !self.match_current(&TokenType::RightBrace) {
                    self.consume(
                        TokenType::Comma,
                        "Expected ',' after field value in struct declaration",
                    )?;
                }
            }

            self.consume(
                TokenType::Semicolon,
                "Expected ';' after struct declaration",
            )?;
            Ok(Stmt::StructInit(StructInitStmt {
                name,
                struct_name,
                arguments,
            }))
        } else {
            let initialiser = self.expr()?;
            self.consume(TokenType::Semicolon, "Expected ';' after let statement")?;
            Ok(Stmt::Let(LetStmt { name, initialiser }))
        }
    }

    fn if_stmt(&mut self) -> ParserResult<Stmt> {
        trace!("Parsing if stmt");
        self.consume(TokenType::LeftParen, "Expected '(' after 'if'")?;
        let condition = self.expr()?;
        self.consume(TokenType::RightParen, "Expected ')' after if expression")?;

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
        let value = self.expr()?;
        self.consume(TokenType::Semicolon, "Expected ';' after print statement")?;
        Ok(Stmt::Print(PrintStmt { value }))
    }

    fn return_stmt(&mut self) -> ParserResult<Stmt> {
        trace!("Parsing return stmt");
        let mut stmt = ReturnStmt { value: None };
        if !self.match_current(&TokenType::Semicolon) {
            stmt.value = Some(self.expr()?);
        }
        self.consume(TokenType::Semicolon, "Expected ';' after return statement")?;
        trace!("Parser::return_stmt return_value = {:?}", &stmt.value);
        Ok(Stmt::Return(stmt))
    }

    fn expression_stmt(&mut self) -> ParserResult<Stmt> {
        trace!("Parsing expression stmt");
        let expr = self.expr()?;
        self.consume(TokenType::Semicolon, "Expected ';' after expression")?;
        Ok(Stmt::Expression(expr))
    }
}
