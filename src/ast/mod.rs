use log::error;
mod token;

use crate::die;

pub use token::{LiteralValue, Token, TokenType};

#[derive(Debug, Clone, Copy)]
pub enum BinOp {
    OpAdd,          // +
    OpSub,          // -
    OpMul,          // *
    OpDiv,          // /
    OpMod,          // %
    OpLess,         // <
    OpLessEqual,    // <=
    OpGreater,      // >
    OpGreaterEqual, // >=
    OpEqualEqual,   // ==
    OpBangEqual,    // !=
    OpAnd,          // and
    OpOr,           // or
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
            TokenType::Less => BinOp::OpLess,
            TokenType::LessEqual => BinOp::OpLessEqual,
            TokenType::Greater => BinOp::OpGreater,
            TokenType::GreaterEqual => BinOp::OpGreaterEqual,
            TokenType::EqualEqual => BinOp::OpEqualEqual,
            TokenType::BangEqual => BinOp::OpBangEqual,
            TokenType::And => BinOp::OpAnd,
            TokenType::Or => BinOp::OpOr,
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
pub struct CallExpr {
    pub callee: Expr,
    pub arg: Option<Expr>,
}

#[derive(Debug, Clone)]
pub struct FieldAccessExpr {
    pub parent: Expr,
    pub field: String,
}

#[derive(Debug, Clone)]
pub struct NativeCallExpr {
    pub callee: Expr,
    pub args: Vec<Expr>,
}

#[derive(Debug, Clone)]
pub struct VariableExpr {
    pub name: String,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Binary(Box<BinExpr>),
    Call(Box<CallExpr>),
    FieldAccess(Box<FieldAccessExpr>),
    NativeCall(Box<NativeCallExpr>),
    Unary(Box<UnaryExpr>),
    Grouping(Box<GroupExpr>),
    Variable(Box<VariableExpr>),
    Literal(LiteralExpr),
}

#[derive(Debug, Clone)]
pub struct BlockStmt {
    pub stmts: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub struct LetStmt {
    pub name: String,
    pub initialiser: Expr,
}

#[derive(Debug, Clone)]
pub struct StructInitStmt {
    pub name: String,
    pub struct_name: String,
    pub arguments: Vec<Expr>,
}

#[derive(Debug, Clone)]
pub struct IfStmt {
    pub condition: Expr,
    pub if_branch: Stmt,
    pub else_branch: Option<Stmt>,
}

#[derive(Debug, Clone)]
pub struct PrintStmt {
    pub value: Expr,
}

#[derive(Debug, Clone)]
pub struct ReturnStmt {
    pub value: Expr,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Block(BlockStmt),
    Let(LetStmt),
    StructInit(StructInitStmt),
    If(Box<IfStmt>),
    Print(PrintStmt),
    Return(ReturnStmt),
    Expression(Expr),
}

#[derive(Debug, Clone)]
pub struct StructDecl {
    pub name: String,
    pub fields: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct FnDecl {
    pub name: String,
    pub parameter: Option<String>,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub enum TopLevelDecl {
    Struct(StructDecl),
    Fn(FnDecl),
}
