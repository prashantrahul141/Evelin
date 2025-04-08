use anyhow::bail;

use crate::ast::{
    TokenType, {FnDecl, Stmt, StructDecl},
};

use super::{Parser, ParserResult};

impl Parser<'_> {
    pub(super) fn fn_decl(&mut self) -> ParserResult<FnDecl> {
        let name = self
            .consume(TokenType::Identifier, "Expected function name")?
            .lexeme
            .clone();

        self.consume(TokenType::LeftParen, "Expected '(' after function name")?;
        let mut parameter = None;
        if !self.match_current(&TokenType::RightParen) {
            parameter = Some(self.advance().lexeme.clone());
        }

        dbg!(&parameter);

        self.consume(
            TokenType::RightParen,
            "Expected ')' after function parameter",
        )?;

        self.consume(
            TokenType::LeftBrace,
            "Expected '{' after function parameter",
        )?;

        let body = match self.block()? {
            Stmt::Block(block) => block.stmts,
            _ => bail!("Expected block after function declaration"),
        };

        Ok(FnDecl {
            name,
            parameter,
            body,
        })
    }

    pub(super) fn struct_decl(&mut self) -> ParserResult<StructDecl> {
        let name = self
            .consume(TokenType::Identifier, "Expected struct name")?
            .lexeme
            .clone();

        self.consume(TokenType::LeftBrace, "Expected '{' after struct name")?;

        let mut fields = vec![];
        while !self.match_token(&[TokenType::RightBrace]) && !self.is_at_end() {
            let field = self
                .consume(TokenType::Identifier, "Expected field name")?
                .lexeme
                .clone();

            fields.push(field);

            if !self.match_current(&TokenType::RightBrace) {
                self.consume(TokenType::Comma, "Expected ',' after field name")?;
            }
        }

        Ok(StructDecl { name, fields })
    }
}
