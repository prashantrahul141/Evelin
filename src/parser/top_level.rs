use anyhow::bail;

use crate::ast::{DType, FnDecl, FnStDeclField, Stmt, StructDecl, TokenType};

use super::{Parser, ParserResult};

impl Parser<'_> {
    pub(super) fn fn_decl(&mut self) -> ParserResult<FnDecl> {
        let name = self
            .consume(TokenType::Identifier, "Expected function name")?
            .lexeme
            .clone();

        self.consume(TokenType::LeftParen, "Expected '(' after function name")?;
        let mut parameter = None;
        if self.match_current(&TokenType::Identifier) {
            let field_name = self.advance().lexeme.clone();
            self.consume(TokenType::Colon, "Expected ':' after function parameter")?;

            let field_type = if self.current().is_a_basic_type() {
                DType::Primitive(self.advance().clone())
            } else {
                DType::Derived(self.advance().lexeme.clone())
            };

            parameter = Some(FnStDeclField {
                field_name,
                field_type,
            });
        }

        self.consume(
            TokenType::RightParen,
            "Expected ')' after function parameter",
        )?;

        self.consume(
            TokenType::FatArrow,
            "Expected '->' after function parameter",
        )?;

        if !self.current().is_a_type() {
            bail!("Expected function return type");
        }

        let return_type = self.advance().clone();

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
            return_type,
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
            let field_name = self
                .consume(TokenType::Identifier, "Expected field name")?
                .lexeme
                .clone();

            self.consume(TokenType::Colon, "Expected ':' after field name")?;

            let field_type = if self.current().is_a_basic_type() {
                DType::Primitive(self.advance().clone())
            } else {
                DType::Derived(self.advance().lexeme.clone())
            };

            fields.push(FnStDeclField {
                field_name,
                field_type,
            });

            if !self.match_current(&TokenType::RightBrace) {
                self.consume(TokenType::Comma, "Expected ',' after field name")?;
            }
        }

        Ok(StructDecl { name, fields })
    }
}
