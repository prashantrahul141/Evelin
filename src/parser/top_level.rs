use anyhow::anyhow;

use crate::ast::{
    TokenType, {FnDecl, Stmt, StructDecl},
};

use super::{Parser, ParserResult};

impl<'a> Parser<'a> {
    pub(super) fn fn_decl(&mut self) -> ParserResult<FnDecl> {
        let name = match self.consume(TokenType::Identifier, "Expected function name.") {
            Some(v) => v.lexeme.clone(),
            None => return Err(anyhow!("Expected function name,")),
        };

        self.consume(TokenType::LeftParen, "Expected '(' after function name");
        let mut parameter = None;
        if !self.match_current(&TokenType::RightParen) {
            parameter = Some(self.advance().lexeme.clone());
        }

        dbg!(&parameter);

        self.consume(
            TokenType::RightParen,
            "Expected ')' after function parameter",
        );

        self.consume(TokenType::LeftBrace, "Expected { after function parameter");
        let body = match self.block()? {
            Stmt::Block(block) => block.stmts,
            _ => return Err(anyhow!("Expected block after function declaration")),
        };

        Ok(FnDecl {
            name,
            parameter,
            body,
        })
    }

    pub(super) fn struct_decl(&mut self) -> ParserResult<StructDecl> {
        let name = match self.consume(TokenType::Identifier, "Expected struct name") {
            Some(v) => v.lexeme.clone(),
            None => return Err(anyhow!("Expected struct name")),
        };

        self.consume(TokenType::LeftBrace, "Expected '{' after struct name");

        let mut fields = vec![];
        while !self.match_token(&[TokenType::RightBrace]) && !self.is_at_end() {
            match self.consume(TokenType::Identifier, "Expected field name") {
                Some(f) => fields.push(f.lexeme.clone()),
                None => return Err(anyhow!("Expected field name")),
            };

            if !self.match_current(&TokenType::RightBrace) {
                self.consume(TokenType::Comma, "Expected commo after field name");
            }
        }

        Ok(StructDecl { name, fields })
    }
}
