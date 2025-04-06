use super::{MAX_NATIVE_FUNCTION_ARITY, ParserResult, parser::Parser};

use anyhow::anyhow;
use log::{error, trace};

use crate::{
    ast::{
        BinExpr, BinOp, CallExpr, Expr, FieldAccessExpr, GroupExpr, LiteralExpr, NativeCallExpr,
        UnOp, UnaryExpr, VariableExpr,
    },
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
        let mut callee = self.primary();
        trace!(
            "Parser::primary before parsing trailing functions = {:?}",
            &callee
        );

        // parses trailing function calls
        loop {
            if self.match_token(&[TokenType::LeftParen]) {
                callee = self.native_finish_call(callee);
                trace!("calle: {:?}", &callee);
            } else {
                break;
            }
        }

        callee
    }

    /// Parses trailing native function calls and function arguments.
    fn native_finish_call(&mut self, callee: ParserResult<Expr>) -> ParserResult<Expr> {
        if let Ok(callee) = callee {
            let mut args = vec![];
            if !self.match_current(&TokenType::RightParen) {
                loop {
                    if args.len() >= MAX_NATIVE_FUNCTION_ARITY {
                        error!("parsing function exceded the MAX_NATIVE_FUNCTION_ARITY limit");
                    }

                    if let Ok(arg) = self.expr() {
                        args.push(arg);
                    }

                    if !self.match_token(&[TokenType::Comma]) {
                        break;
                    }
                }
            }

            if let Some(_) = self.consume(
                TokenType::RightParen,
                "Expected ')' after function arguments.",
            ) {
                let call = Expr::NativeCall(Box::new(NativeCallExpr { callee, args }));
                return Ok(call);
            }
        }

        Err(anyhow!("Failed to parse native function."))
    }

    /// Parses function calling expressions.
    fn call(&mut self) -> ParserResult<Expr> {
        trace!("Parsing call");
        let mut callee = self.primary();
        trace!(
            "Parser::primary before parsing trailing functions = {:?}",
            &callee
        );

        // parses trailing function calls
        loop {
            if self.match_token(&[TokenType::LeftParen]) {
                callee = self.finish_call(callee);
            } else if self.match_token(&[TokenType::Dot]) {
                callee = self.finish_access(callee);
            } else {
                break;
            }
            trace!("calle: {:?}", &callee);
        }

        callee
    }

    /// Parses trailing function calls and function argument.
    fn finish_call(&mut self, callee: ParserResult<Expr>) -> ParserResult<Expr> {
        if let Ok(mut callee) = callee {
            if !self.match_current(&TokenType::RightParen) {
                trace!("parsing function argument.");
                if let Ok(arg) = self.expr() {
                    callee = Expr::Call(Box::new(CallExpr {
                        callee,
                        arg: Some(arg),
                    }));
                }
            } else {
                callee = Expr::Call(Box::new(CallExpr { callee, arg: None }));
            }

            self.consume(
                TokenType::RightParen,
                "Expected ')' after function argument.\n\nNote: Multiple function arguments are not supported, use structs for that.",
            );

            return Ok(callee);
        }

        let error = self.report_parser_error("Failed to parse function call expression.");
        Err(anyhow!(error))
    }

    /// parses trailing field access.
    fn finish_access(&mut self, callee: ParserResult<Expr>) -> ParserResult<Expr> {
        if let Ok(callee) = callee {
            let field = match self.consume(TokenType::Identifier, "Expected field name") {
                Some(v) => v.lexeme.clone(),
                None => return Err(anyhow!("Expected field name")),
            };
            return Ok(Expr::FieldAccess(Box::new(FieldAccessExpr {
                parent: callee,
                field,
            })));
        }

        let error = self.report_parser_error("Failed to parse trailing field access.");
        Err(anyhow!(error))
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
            if let Ok(expr) = self.expr() {
                self.consume(
                    TokenType::RightParen,
                    format!("Expected ')' got {} instead", self.current().ttype).as_str(),
                );
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

        let error = self.report_parser_error(format!(
            "Expected literal value recieved {} instead.",
            self.current().ttype
        ));

        error!(
            "Expected literal value recieved {} instead.",
            self.current()
        );

        Err(anyhow!(error))
    }
}
