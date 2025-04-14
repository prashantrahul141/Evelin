use crate::ast::{
    BinExpr, BinOp, CallExpr, Expr, FieldAccessExpr, GroupExpr, LiteralExpr, LiteralValue,
    NativeCallExpr, UnOp, UnaryExpr, VariableExpr,
};
use crate::emitter::EmitterResult;
use anyhow::bail;
use log::{error, trace};
use qbe;

use super::QBEEmitter;

impl QBEEmitter<'_> {
    /// Emit expr
    pub(super) fn emit_expr(
        &mut self,
        func: &mut qbe::Function<'static>,
        expr: &Expr,
    ) -> EmitterResult<(qbe::Type<'static>, qbe::Value)> {
        trace!("emitting expr = {:?}", expr);
        match expr {
            Expr::Binary(bin) => self.emit_binary(func, bin),
            Expr::Call(call) => self.emit_call(func, call),
            Expr::FieldAccess(fiac) => self.emit_field_access(func, fiac),
            Expr::NativeCall(call) => self.emit_native_call(func, call),
            Expr::Unary(una) => self.emit_unary(func, una),
            Expr::Grouping(gro) => self.emit_grouping(func, gro),
            Expr::Literal(lit) => self.emit_literal(func, lit),
            Expr::Variable(var) => self.emit_variable(var),
        }
    }

    /// Emit binary operation ast.
    fn emit_binary(
        &mut self,
        func: &mut qbe::Function<'static>,
        expr: &BinExpr,
    ) -> EmitterResult<(qbe::Type<'static>, qbe::Value)> {
        trace!("emitting binary expr = {:?}", expr);
        let (ty_left, left) = self.emit_expr(func, &expr.left)?;
        let (ty_right, right) = self.emit_expr(func, &expr.right)?;
        let tmp = self.new_tmp();

        let mut ty = qbe::Type::Word;
        if matches!(ty_left, qbe::Type::Single) || matches!(ty_right, qbe::Type::Single) {
            ty = qbe::Type::Single;
        }

        func.assign_instr(
            tmp.clone(),
            ty.clone(),
            match expr.op {
                // arithmetic
                BinOp::Add => qbe::Instr::Add(left, right),
                BinOp::Sub => qbe::Instr::Sub(left, right),
                BinOp::Mul => qbe::Instr::Mul(left, right),
                BinOp::Div => qbe::Instr::Div(left, right),
                BinOp::Mod => qbe::Instr::Rem(left, right),

                // logical
                BinOp::Or => qbe::Instr::Or(left, right),
                BinOp::And => qbe::Instr::And(left, right),

                // comparison
                cmp => qbe::Instr::Cmp(
                    ty.clone(),
                    match cmp {
                        BinOp::Less => qbe::Cmp::Slt,
                        BinOp::LessEqual => qbe::Cmp::Sle,
                        BinOp::Greater => qbe::Cmp::Sgt,
                        BinOp::GreaterEqual => qbe::Cmp::Sge,
                        BinOp::EqualEqual => qbe::Cmp::Eq,
                        BinOp::BangEqual => qbe::Cmp::Ne,
                        _ => unreachable!("binop"),
                    },
                    left,
                    right,
                ),
            },
        );

        Ok((ty, tmp))
    }

    /// Emit Eve function call
    fn emit_call(
        &mut self,
        func: &mut qbe::Function<'static>,
        call: &CallExpr,
    ) -> EmitterResult<(qbe::Type<'static>, qbe::Value)> {
        trace!("emitting call expr call = {:?}", call);
        let ty = qbe::Type::Word;
        let arg = if let Some(arg_expr) = &call.arg {
            vec![self.emit_expr(func, arg_expr)?]
        } else {
            vec![]
        };

        let tmp = self.new_tmp();

        if let Expr::Variable(var) = &call.callee {
            func.assign_instr(
                tmp.clone(),
                ty.clone(),
                qbe::Instr::Call(var.name.clone(), arg, None),
            );
        } else {
            error!("Expected function name got '{:?}' instead", call.callee);
            bail!("Expected function name got '{:?}' instead", call.callee);
        }

        Ok((ty, tmp))
    }

    /// Emits struct field access
    fn emit_field_access(
        &self,
        func: &mut qbe::Function<'static>,
        fiac: &FieldAccessExpr,
    ) -> Result<(qbe::Type<'static>, qbe::Value), anyhow::Error> {
        todo!()
    }

    /// Emit eve native function call
    fn emit_native_call(
        &mut self,
        func: &mut qbe::Function<'static>,
        call: &NativeCallExpr,
    ) -> EmitterResult<(qbe::Type<'static>, qbe::Value)> {
        trace!("emitting native call expr call = {:?}", call);
        let ty = qbe::Type::Word;
        let args = call
            .args
            .iter()
            .map(|arg| self.emit_expr(func, arg))
            .collect::<Result<Vec<_>, _>>()?;

        let tmp = self.new_tmp();

        if let Expr::Variable(var) = &call.callee {
            func.assign_instr(
                tmp.clone(),
                ty.clone(),
                qbe::Instr::Call(var.name.clone(), args, None),
            );
        } else {
            error!("Expected function name got '{:?}' instead", call.callee);
            bail!("Expected function name got '{:?}' instead", call.callee);
        }

        Ok((ty, tmp))
    }

    /// Emit unary operation ast.
    fn emit_unary(
        &mut self,
        func: &mut qbe::Function<'static>,
        expr: &UnaryExpr,
    ) -> EmitterResult<(qbe::Type<'static>, qbe::Value)> {
        trace!("emitting unary expr = {:?}", expr);
        let tmp = self.new_tmp();
        let (ty, operand) = self.emit_expr(func, &expr.operand)?;

        func.assign_instr(
            tmp.clone(),
            ty.clone(),
            match expr.op {
                UnOp::OpSub => qbe::Instr::Copy(operand),
                UnOp::OpNeg => todo!(),
            },
        );

        Ok((ty, tmp))
    }

    /// Emits grouping ast.
    fn emit_grouping(
        &mut self,
        func: &mut qbe::Function<'static>,
        expr: &GroupExpr,
    ) -> EmitterResult<(qbe::Type<'static>, qbe::Value)> {
        trace!("emitting grouping expr = {:?}", expr);
        let tmp = self.new_tmp();
        let (ty, value) = self.emit_expr(func, &expr.value)?;
        func.assign_instr(tmp.clone(), ty.clone(), qbe::Instr::Copy(value));
        Ok((ty, tmp))
    }

    /// Emits variable expression
    fn emit_variable(
        &mut self,
        expr: &VariableExpr,
    ) -> EmitterResult<(qbe::Type<'static>, qbe::Value)> {
        trace!("emitting variable expr = {:?}", expr);
        let (ty, tmp) = self.get_var(&expr.name)?.clone();
        Ok((ty, tmp))
    }

    /// Emits literal values in form of temporaries.
    fn emit_literal(
        &mut self,
        func: &mut qbe::Function<'static>,
        expr: &LiteralExpr,
    ) -> EmitterResult<(qbe::Type<'static>, qbe::Value)> {
        trace!("emitting literal expr = {:?}", expr);
        let v = &expr.value;
        match v {
            LiteralValue::NumberFloat(v) => {
                let tmp = self.new_tmp();
                let ty = qbe::Type::Single;
                func.assign_instr(
                    tmp.clone(),
                    ty.clone(),
                    qbe::Instr::Copy(qbe::Value::Const(*v as u64)),
                );

                Ok((ty, tmp))
            }
            LiteralValue::NumberInt(v) => {
                let tmp = self.new_tmp();
                let ty = qbe::Type::Word;
                func.assign_instr(
                    tmp.clone(),
                    ty.clone(),
                    qbe::Instr::Copy(qbe::Value::Const(*v as u64)),
                );

                Ok((ty, tmp))
            }
            LiteralValue::String(v) => {
                let tmp = self.new_tmp();
                let ty = qbe::Type::Long;
                let glob_name = self.new_glob_name();
                let def = self.module.add_data(qbe::DataDef::new(
                    qbe::Linkage::private(),
                    glob_name,
                    None,
                    vec![
                        (qbe::Type::Byte, qbe::DataItem::Str(v.into())),
                        (qbe::Type::Byte, qbe::DataItem::Const(0)),
                    ],
                ));

                func.assign_instr(
                    tmp.clone(),
                    ty.clone(),
                    qbe::Instr::Copy(qbe::Value::Global(def.name.clone())),
                );

                Ok((ty, tmp))
            }

            LiteralValue::Boolean(_) => todo!(),
            LiteralValue::Null => todo!(),
        }
    }
}
