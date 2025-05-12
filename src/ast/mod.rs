use std::ops::{Deref, DerefMut};

use anyhow::bail;
use log::error;
mod token;

use crate::die;

pub use token::{LiteralValue, Token, TokenType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Metadata {
    pub line: usize,
    pub node_type: Option<DType>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EveTypes {
    Int,
    Float,
    String,
    Void,
}

impl std::fmt::Display for EveTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                EveTypes::Int => "Int",
                EveTypes::Float => "Float",
                EveTypes::String => "String",
                EveTypes::Void => "Void",
            }
        )
    }
}

impl TryFrom<&Token> for EveTypes {
    type Error = anyhow::Error;

    fn try_from(value: &Token) -> Result<Self, Self::Error> {
        match &value.ttype {
            TokenType::TypeInt => Ok(EveTypes::Int),
            TokenType::TypeFloat => Ok(EveTypes::Float),
            TokenType::TypeVoid => Ok(EveTypes::Void),
            TokenType::String => Ok(EveTypes::String),
            ty => bail!("EveTypes::TryFrom<Token>  recieved type = {}", ty),
        }
    }
}

impl TryFrom<&DType> for EveTypes {
    type Error = anyhow::Error;

    fn try_from(value: &DType) -> Result<Self, Self::Error> {
        match &value {
            DType::Primitive(e) => Ok(e.to_owned()),
            DType::Derived(_) => bail!("EveTypes::TryFrom<DType> recieved derived type"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Clone, PartialEq)]
pub struct BinExpr {
    pub left: Expr,
    pub op: BinOp,
    pub right: Expr,
    pub metadata: Metadata,
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

#[derive(Debug, Clone, PartialEq)]
pub enum UnOp {
    OpSub,  // -
    OpFact, // !
}

impl std::fmt::Display for UnOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                UnOp::OpSub => "-",
                UnOp::OpFact => "!",
            },
        )
    }
}

impl From<&TokenType> for UnOp {
    fn from(value: &TokenType) -> Self {
        match value {
            TokenType::Minus => UnOp::OpSub,
            TokenType::Bang => UnOp::OpFact,
            _ => {
                die!("UnOp::from failed recieved: {}", value);
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnaryExpr {
    pub op: UnOp,
    pub operand: Expr,
    pub metadata: Metadata,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LiteralExpr {
    pub value: LiteralValue,
    pub metadata: Metadata,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GroupExpr {
    pub value: Expr,
    pub metadata: Metadata,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CallExpr {
    pub callee: Expr,
    pub arg: Option<Expr>,
    pub metadata: Metadata,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FieldAccessExpr {
    pub parent: Expr,
    pub field: String,
    pub metadata: Metadata,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NativeCallExpr {
    pub callee: Expr,
    pub args: Vec<Expr>,
    pub metadata: Metadata,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VariableExpr {
    pub name: String,
    pub metadata: Metadata,
}

#[derive(Debug, Clone, PartialEq)]
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

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Binary(bin) => write!(f, "{} {} {}", bin.left, bin.op, bin.right),
            Expr::Call(call) => match &call.arg {
                Some(arg) => write!(f, "{}({})", call.callee, arg),
                None => write!(f, "{}()", call.callee),
            },
            Expr::FieldAccess(fac) => write!(f, "{}.{}", fac.parent, fac.field),
            Expr::NativeCall(call) => write!(
                f,
                "{}({})",
                call.callee,
                call.args
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Expr::Unary(un) => write!(f, "{}{}", un.op, un.operand),
            Expr::Grouping(gr) => write!(f, "({})", gr.value),
            Expr::Variable(var) => write!(f, "{}", var.name),
            Expr::Literal(lit) => write!(f, "{}", lit.value),
        }
    }
}

impl Deref for Expr {
    type Target = Metadata;
    fn deref(&self) -> &Self::Target {
        match self {
            Expr::Binary(bin) => &bin.metadata,
            Expr::Call(call) => &call.metadata,
            Expr::FieldAccess(fieldacc) => &fieldacc.metadata,
            Expr::NativeCall(nativecall) => &nativecall.metadata,
            Expr::Unary(unary) => &unary.metadata,
            Expr::Grouping(group) => &group.metadata,
            Expr::Variable(var) => &var.metadata,
            Expr::Literal(lit) => &lit.metadata,
        }
    }
}

impl DerefMut for Expr {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Expr::Binary(bin) => &mut bin.metadata,
            Expr::Call(call) => &mut call.metadata,
            Expr::FieldAccess(fieldacc) => &mut fieldacc.metadata,
            Expr::NativeCall(nativecall) => &mut nativecall.metadata,
            Expr::Unary(unary) => &mut unary.metadata,
            Expr::Grouping(group) => &mut group.metadata,
            Expr::Variable(var) => &mut var.metadata,
            Expr::Literal(lit) => &mut lit.metadata,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BlockStmt {
    pub stmts: Vec<Stmt>,
    pub metadata: Metadata,
}

#[derive(Debug, Clone)]
pub struct LetStmt {
    pub name: String,
    pub initialiser: Expr,
    pub metadata: Metadata,
}

#[derive(Debug, Clone)]
pub struct StInitField {
    pub field_name: String,
    pub field_expr: Expr,
    pub metadata: Metadata,
}

#[derive(Debug, Clone)]
pub struct StructInitStmt {
    pub name: String,
    pub struct_name: String,
    pub arguments: Vec<StInitField>,
    pub metadata: Metadata,
}

#[derive(Debug, Clone)]
pub struct IfStmt {
    pub condition: Expr,
    pub if_branch: Stmt,
    pub else_branch: Option<Stmt>,
    pub metadata: Metadata,
}

#[derive(Debug, Clone)]
pub struct PrintStmt {
    pub value: Expr,
    pub metadata: Metadata,
}

#[derive(Debug, Clone)]
pub struct ReturnStmt {
    pub value: Option<Expr>,
    pub metadata: Metadata,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DType {
    Primitive(EveTypes),
    Derived(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FnStDeclField {
    pub field_name: String,
    // This is only Primitive in case of structs for now.
    pub field_type: DType,
    pub metadata: Metadata,
}

#[derive(Debug, Clone)]
pub struct StructDecl {
    pub name: String,
    pub fields: Vec<FnStDeclField>,
    pub metadata: Metadata,
}

#[derive(Debug, Clone)]
pub struct FnDecl {
    pub name: String,
    pub parameter: Option<FnStDeclField>,
    pub return_type: DType,
    pub body: Vec<Stmt>,
    pub metadata: Metadata,
}
