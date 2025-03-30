use crate::token::LiteralValue;

#[derive(Debug)]
pub enum BinOp {
    OpAdd, // +
    OpSub, // -
    OpMul, // *
    OpDiv, // /
    OpMod, // %
}

#[derive(Debug)]
pub enum UnOp {
    OpSub, // -
}

#[derive(Debug)]
pub struct BinExpr {
    left: Expr,
    op: BinOp,
    right: Expr,
}

#[derive(Debug)]
pub struct UnaryExpr {
    op: UnOp,
    operand: Expr,
}

#[derive(Debug)]
pub struct LiteralExpr {
    value: LiteralValue,
}

#[derive(Debug)]
pub struct GroupExpr {
    value: Expr,
}

#[derive(Debug)]
pub enum Expr {
    Binary(Box<BinExpr>),
    Unary(Box<UnaryExpr>),
    Grouping(Box<GroupExpr>),
    Literal(LiteralExpr),
}
