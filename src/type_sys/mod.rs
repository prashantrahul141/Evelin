use anyhow::bail;

use crate::{
    ast::{BinExpr, CallExpr, EveTypes, Expr, FnDecl, LiteralExpr, LiteralValue, StructDecl},
    utils::{ErrorType, MessageType, report_message},
};

/// Anotate ast with types and check them.
pub struct TypeSystem<'a> {
    fn_decls: &'a Vec<FnDecl>,
    st_decls: &'a Vec<StructDecl>,
    pub errors_count: usize,
}

impl<'a> TypeSystem<'a> {
    pub fn new(fn_decls: &'a Vec<FnDecl>, st_decls: &'a Vec<StructDecl>) -> Self {
        Self {
            fn_decls,
            st_decls,
            errors_count: 0,
        }
    }

    pub fn check(&self) {}

    fn check_expr(&self, expr: &Expr) -> anyhow::Result<EveTypes> {
        Ok(match expr {
            Expr::Binary(bin) => self.check_binary(bin)?,
            Expr::Call(call) => self.check_call(call)?,
            Expr::FieldAccess(_) => todo!(),
            Expr::NativeCall(_) => todo!(),
            Expr::Unary(_) => todo!(),
            Expr::Grouping(group) => self.check_expr(&group.value)?,
            Expr::Variable(_) => todo!(),
            Expr::Literal(lit) => self.check_literal(lit)?,
        })
    }

    fn check_binary(&self, bin: &BinExpr) -> anyhow::Result<EveTypes> {
        let left = self.check_expr(&bin.left)?;
        let right = self.check_expr(&bin.right)?;

        match (left, right) {
            (EveTypes::Int, EveTypes::Int) => Ok(EveTypes::Int),
            (EveTypes::Int, EveTypes::Float) => Ok(EveTypes::Float),
            (EveTypes::Int, EveTypes::String) => bail!(
                "{} operation cannot be applied between int and string",
                &bin.op,
            ),
            (EveTypes::Float, EveTypes::Int) => Ok(EveTypes::Float),
            (EveTypes::Float, EveTypes::Float) => Ok(EveTypes::Float),
            (EveTypes::Float, EveTypes::String) => bail!(
                "{} operation cannot be applied between float and string",
                &bin.op,
            ),
            (EveTypes::String, EveTypes::Int) => bail!(
                "{} operation cannot be applied between string and int",
                &bin.op,
            ),
            (EveTypes::String, EveTypes::Float) => bail!(
                "{} operation cannot be applied between string and float",
                &bin.op,
            ),
            (EveTypes::String, EveTypes::String) => bail!(
                "{} operation cannot be applied between string and string",
                &bin.op,
            ),
        }
    }

    fn check_call(&self, call: &CallExpr) -> anyhow::Result<EveTypes> {
        bail!("l")
    }

    fn check_literal(&self, literal: &LiteralExpr) -> anyhow::Result<EveTypes> {
        Ok(match literal.value {
            LiteralValue::NumberFloat(_) => EveTypes::Float,
            LiteralValue::NumberInt(_) => EveTypes::Int,
            LiteralValue::String(_) => EveTypes::String,
            LiteralValue::Boolean(_) => EveTypes::Int,
            LiteralValue::Null => EveTypes::Int,
        })
    }

    fn get_fn_from_name<S: Into<String>>(&self, fn_name: S) -> anyhow::Result<FnDecl> {
        todo!()
    }

    fn report_msg<M: Into<String>>(msg: M) {
        report_message(msg.into(), MessageType::Error(ErrorType::TypeError))
    }
}
