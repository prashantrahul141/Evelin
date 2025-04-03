use super::{ParserResult, parser::Parser};

use anyhow::anyhow;
use log::trace;

use crate::{
    ast::{BinExpr, BinOp, Expr, GroupExpr, LiteralExpr, UnOp, UnaryExpr},
    token::{LiteralValue, TokenType},
};

impl<'a> Parser<'a> {
    /// Parses top-level expressions.
    pub(super) fn expr(&mut self) -> ParserResult<Expr> {
        self.or()
    }

    /// parses logical or expressions
    fn or(&mut self) -> ParserResult<Expr> {
        let mut left = self.and()?;
        while self.match_token(&[TokenType::Or]) {
            let op = BinOp::from(&self.previous().ttype);
            let right = self.and()?;
            let bin = BinExpr {
                left: left.clone(),
                op,
                right,
            };
            left = Expr::Binary(Box::new(bin));
        }

        Ok(left)
    }

    /// parses logical and expressions
    fn and(&mut self) -> ParserResult<Expr> {
        let mut left = self.equality()?;
        while self.match_token(&[TokenType::And]) {
            let op = BinOp::from(&self.previous().ttype);
            let right = self.equality()?;
            let bin = BinExpr {
                left: left.clone(),
                op,
                right,
            };
            left = Expr::Binary(Box::new(bin));
        }

        Ok(left)
    }

    /// Parses equality expressions.
    fn equality(&mut self) -> ParserResult<Expr> {
        let mut left = self.comparsion()?;
        while self.match_token(&[TokenType::EqualEqual, TokenType::BangEqual]) {
            let op = BinOp::from(&self.previous().ttype);
            let right = self.comparsion()?;
            let bin = BinExpr {
                left: left.clone(),
                op,
                right,
            };
            left = Expr::Binary(Box::new(bin));
        }

        Ok(left)
    }

    /// Parses comparsion expressions.
    fn comparsion(&mut self) -> ParserResult<Expr> {
        let mut left = self.term()?;
        while self.match_token(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let op = BinOp::from(&self.previous().ttype);
            let right = self.term()?;
            let bin = BinExpr {
                left: left.clone(),
                op,
                right,
            };
            left = Expr::Binary(Box::new(bin));
        }

        Ok(left)
    }

    /// Parses term expressions.
    fn term(&mut self) -> ParserResult<Expr> {
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
    fn factor(&mut self) -> ParserResult<Expr> {
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
    fn unary(&mut self) -> ParserResult<Expr> {
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
    fn primary(&mut self) -> ParserResult<Expr> {
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
}
