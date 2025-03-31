use log::error;

use crate::{
    die,
    token::{LiteralValue, TokenType},
};

#[derive(Debug, Clone, Copy)]
pub enum BinOp {
    OpAdd, // +
    OpSub, // -
    OpMul, // *
    OpDiv, // /
    OpMod, // %
}

#[derive(Debug, Clone)]
pub struct BinExpr {
    pub left: Expr,
    pub op: BinOp,
    pub right: Expr,
}

impl From<&TokenType> for BinOp {
    fn from(value: &TokenType) -> Self {
        match value {
            TokenType::Plus => BinOp::OpAdd,
            TokenType::Minus => BinOp::OpSub,
            TokenType::Slash => BinOp::OpDiv,
            TokenType::Star => BinOp::OpMul,
            TokenType::Mod => BinOp::OpMod,
            _ => {
                die!("BinOp::from failed recieved: {}", value);
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum UnOp {
    OpSub, // -
    OpNeg, // !
}

impl From<&TokenType> for UnOp {
    fn from(value: &TokenType) -> Self {
        match value {
            TokenType::Minus => UnOp::OpSub,
            TokenType::Bang => UnOp::OpNeg,
            _ => {
                die!("UnOp::from failed recieved: {}", value);
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct UnaryExpr {
    pub op: UnOp,
    pub operand: Expr,
}

#[derive(Debug, Clone)]
pub struct LiteralExpr {
    pub value: LiteralValue,
}

#[derive(Debug, Clone)]
pub struct GroupExpr {
    pub value: Expr,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Binary(Box<BinExpr>),
    Unary(Box<UnaryExpr>),
    Grouping(Box<GroupExpr>),
    Literal(LiteralExpr),
}
