use log::error;
mod token;

use crate::die;

pub use token::{LiteralValue, Token, TokenType};

#[derive(Debug, Clone, Copy)]
pub enum BinOp {
    Add,          // +
    Sub,          // -
    Mul,          // *
    Div,          // /
    Mod,          // %
    Less,         // <
    LessEqual,    // <=
    Greater,      // >
    GreaterEqual, // >=
    EqualEqual,   // ==
    BangEqual,    // !=
    And,          // and
    Or,           // or
}

#[derive(Debug, Clone)]
pub struct BinExpr {
    pub left: Expr,
    pub op: BinOp,
    pub right: Expr,
}

impl std::fmt::Display for BinOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                BinOp::Add => "+",
                BinOp::Sub => "-",
                BinOp::Mul => "*",
                BinOp::Div => "/",
                BinOp::Mod => "%",
                BinOp::Less => "<",
                BinOp::LessEqual => "<=",
                BinOp::Greater => ">",
                BinOp::GreaterEqual => ">=",
                BinOp::EqualEqual => "==",
                BinOp::BangEqual => "!=",
                BinOp::And => "&&",
                BinOp::Or => "||",
            }
        )
    }
}

impl From<&TokenType> for BinOp {
    fn from(value: &TokenType) -> Self {
        match value {
            TokenType::Plus => BinOp::Add,
            TokenType::Minus => BinOp::Sub,
            TokenType::Slash => BinOp::Div,
            TokenType::Star => BinOp::Mul,
            TokenType::Mod => BinOp::Mod,
            TokenType::Less => BinOp::Less,
            TokenType::LessEqual => BinOp::LessEqual,
            TokenType::Greater => BinOp::Greater,
            TokenType::GreaterEqual => BinOp::GreaterEqual,
            TokenType::EqualEqual => BinOp::EqualEqual,
            TokenType::BangEqual => BinOp::BangEqual,
            TokenType::And => BinOp::And,
            TokenType::Or => BinOp::Or,
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
pub enum EveTypes {
    Int,
    Float,
    String,
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
pub struct StInitField {
    pub field_name: String,
    pub field_expr: Expr,
}

#[derive(Debug, Clone)]
pub struct StructInitStmt {
    pub name: String,
    pub struct_name: String,
    pub arguments: Vec<StInitField>,
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
    pub value: Option<Expr>,
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

#[derive(Debug, Clone, PartialEq)]
pub enum DType {
    Primitive(Token),
    Derived(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FnStDeclField {
    pub field_name: String,
    pub field_type: DType,
}

#[derive(Debug, Clone)]
pub struct StructDecl {
    pub name: String,
    pub fields: Vec<FnStDeclField>,
}

#[derive(Debug, Clone)]
pub struct FnDecl {
    pub name: String,
    pub parameter: Option<FnStDeclField>,
    pub return_type: Token,
    pub body: Vec<Stmt>,
}
