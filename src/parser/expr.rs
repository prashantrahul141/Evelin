use super::{MAX_NATIVE_FUNCTION_ARITY, Parser, ParserResult};

use anyhow::bail;
use log::trace;

use crate::ast::{
    BinExpr, BinOp, CallExpr, Expr, FieldAccessExpr, GroupExpr, LiteralExpr, LiteralValue,
    NativeCallExpr, TokenType, UnOp, UnaryExpr, VariableExpr,
};

impl Parser<'_> {
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
        trace!("Parsing unary");
        if self.match_token(&[TokenType::Bang, TokenType::Minus]) {
            let op = UnOp::from(&self.previous().ttype);
            if let Ok(operand) = self.unary() {
                let un = UnaryExpr { op, operand };
                return Ok(Expr::Unary(Box::new(un)));
            }
        }

        if self.match_token(&[TokenType::Extern]) {
            return self.native_call();
        }

        self.call()
    }

    /// Parses native function calling expression.
    fn native_call(&mut self) -> ParserResult<Expr> {
        trace!("Parsing native call");
        let callee = self.primary()?;

        trace!("Parser::native_call callee_name = {:?}", &callee);
        if self.match_token(&[TokenType::LeftParen]) {
            return self.native_finish_call(callee);
        }

        Ok(callee)
    }

    /// Parses trailing native function calls and function arguments.
    fn native_finish_call(&mut self, callee: Expr) -> ParserResult<Expr> {
        let mut local_call = Box::new(NativeCallExpr {
            callee,
            args: vec![],
        });

        if !self.match_current(&TokenType::RightParen) {
            loop {
                if local_call.args.len() >= MAX_NATIVE_FUNCTION_ARITY {
                    bail!(
                        "parsing function exceeded the MAX_NATIVE_FUNCTION_ARITY limit of {}",
                        MAX_NATIVE_FUNCTION_ARITY
                    );
                }

                local_call.args.push(self.expr()?);

                if !self.match_token(&[TokenType::Comma]) {
                    break;
                }
            }
        }

        self.consume(
            TokenType::RightParen,
            "Expected ')' after function arguments.",
        )?;

        Ok(Expr::NativeCall(local_call))
    }

    /// Parses function calling expressions.
    fn call(&mut self) -> ParserResult<Expr> {
        trace!("Parsing call");
        let callee = self.primary()?;

        trace!("Parser::call callee_name = {:?}", &callee);
        if self.match_token(&[TokenType::LeftParen]) {
            return self.finish_call(callee);
        } else if self.match_token(&[TokenType::Dot]) {
            return self.finish_access(callee);
        }

        Ok(callee)
    }

    /// Parses trailing function calls and function argument.
    fn finish_call(&mut self, callee: Expr) -> ParserResult<Expr> {
        let mut local_call = Box::new(CallExpr { callee, arg: None });

        if !self.match_current(&TokenType::RightParen) {
            trace!("parsing function argument");
            local_call.arg = Some(self.expr()?);
        }

        self.consume(
                TokenType::RightParen,
                "Expected ')' after function argument.\n\nNote: Multiple function arguments are only supported for extern function calls, otherwise use structs."
            )?;

        Ok(Expr::Call(local_call))
    }

    /// parses trailing field access.
    fn finish_access(&mut self, callee: Expr) -> ParserResult<Expr> {
        let field = self.consume(TokenType::Identifier, "Expected field name")?;
        Ok(Expr::FieldAccess(Box::new(FieldAccessExpr {
            parent: callee,
            field: field.lexeme.clone(),
        })))
    }

    /// Parses literal expressions.
    fn primary(&mut self) -> ParserResult<Expr> {
        trace!("Parser::primary current_token = {}", self.current());
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
            if let Ok(expr) = self.expr() {
                self.consume(
                    TokenType::RightParen,
                    format!("Expected ')' got {} instead", self.current().ttype).as_str(),
                )?;
                let literal = Expr::Grouping(Box::new(GroupExpr { value: expr }));
                return Ok(literal);
            }
        }

        // identifier
        if self.match_token(&[TokenType::Identifier]) {
            let var = Expr::Variable(Box::new(VariableExpr {
                name: self.previous().lexeme.clone(),
            }));
            return Ok(var);
        }

        bail!(
            "Expected literal, expression, or identifier received '{}' instead",
            self.current().lexeme
        );
    }
}
