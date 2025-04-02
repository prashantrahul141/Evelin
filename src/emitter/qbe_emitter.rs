use crate::ast::{BinExpr, Expr, GroupExpr, LiteralExpr, UnaryExpr};
use crate::ast::{BinOp, UnOp};
use crate::emitter::EmitterResult;
use crate::token::LiteralValue;
use qbe;

use super::Emitter;

pub struct QBEEmitter<'a> {
    tmp_counter: usize,
    stmts: &'a Vec<Expr>,
    module: qbe::Module<'a>,
}

impl<'a> From<&'a Vec<Expr>> for QBEEmitter<'a> {
    fn from(stmts: &'a Vec<Expr>) -> Self {
        let mut module = qbe::Module::new();
        let items = vec![
            (
                qbe::Type::Byte,
                qbe::DataItem::Str("One and one make %d!\\n".into()),
            ),
            (qbe::Type::Byte, qbe::DataItem::Const(0)),
        ];
        let data = qbe::DataDef::new(qbe::Linkage::private(), "fmt", None, items);
        module.add_data(data);
        Self {
            tmp_counter: 0,
            stmts,
            module,
        }
    }
}

impl<'a> Emitter for QBEEmitter<'a> {
    fn emit_ir(&mut self) -> EmitterResult<String> {
        self.emit();

        Ok(format!("{}", self.module))
    }
}

impl<'a> QBEEmitter<'a> {
    fn emit(&mut self) {
        let mut main_func = qbe::Function::new(
            qbe::Linkage::public(),
            "main",
            Vec::new(),
            Some(qbe::Type::Word),
        );
        main_func.add_block("start");
        for stmt in self.stmts.iter() {
            self.emit_expr(&mut main_func, stmt).unwrap();
        }
        let last_temp = format!("tmp.{}", self.tmp_counter);
        main_func.add_instr(qbe::Instr::Call(
            "printf".into(),
            vec![
                (qbe::Type::Long, qbe::Value::Global("fmt".into())),
                (qbe::Type::Word, qbe::Value::Temporary(last_temp)),
            ],
            Some(1),
        ));
        main_func.add_instr(qbe::Instr::Ret(Some(qbe::Value::Const(0 as u64))));
        self.module.add_function(main_func);
    }

    fn emit_expr(
        &mut self,
        func: &mut qbe::Function<'static>,
        expr: &Expr,
    ) -> EmitterResult<(qbe::Type<'static>, qbe::Value)> {
        match expr {
            Expr::Binary(bin) => self.emit_binary(func, bin),
            Expr::Unary(una) => self.emit_unary(func, una),
            Expr::Grouping(gro) => self.emit_grouping(func, gro),
            Expr::Literal(lit) => self.emit_literal(func, lit),
        }
    }

    fn emit_binary(
        &mut self,
        func: &mut qbe::Function<'static>,
        expr: &Box<BinExpr>,
    ) -> EmitterResult<(qbe::Type<'static>, qbe::Value)> {
        let expr = expr.as_ref();
        let (_, left) = self.emit_expr(func, &expr.left)?;
        let (_, right) = self.emit_expr(func, &expr.right)?;

        let temp = self.new_temp();
        let ty = qbe::Type::Word;

        func.assign_instr(
            temp.clone(),
            ty.clone(),
            match expr.op {
                BinOp::OpAdd => qbe::Instr::Add(left, right),
                BinOp::OpSub => qbe::Instr::Sub(left, right),
                BinOp::OpMul => qbe::Instr::Mul(left, right),
                BinOp::OpDiv => qbe::Instr::Div(left, right),
                BinOp::OpMod => qbe::Instr::Rem(left, right),
            },
        );

        Ok((ty, temp))
    }

    fn emit_unary(
        &mut self,
        func: &mut qbe::Function<'static>,
        expr: &Box<UnaryExpr>,
    ) -> EmitterResult<(qbe::Type<'static>, qbe::Value)> {
        let expr = expr.as_ref();
        let temp = self.new_temp();
        let (_, operand) = self.emit_expr(func, &expr.operand)?;
        let ty = qbe::Type::Word;

        func.assign_instr(
            temp.clone(),
            ty.clone(),
            match expr.op {
                UnOp::OpSub => qbe::Instr::Copy(operand),
                UnOp::OpNeg => todo!(),
            },
        );

        Ok((ty, temp))
    }

    fn emit_grouping(
        &mut self,
        func: &mut qbe::Function<'static>,
        expr: &Box<GroupExpr>,
    ) -> EmitterResult<(qbe::Type<'static>, qbe::Value)> {
        let expr = expr.as_ref();
        let temp = self.new_temp();
        let (_, value) = self.emit_expr(func, &expr.value)?;
        let ty = qbe::Type::Word;
        func.assign_instr(temp.clone(), ty.clone(), qbe::Instr::Copy(value));
        Ok((ty, temp))
    }

    fn emit_literal(
        &mut self,
        func: &mut qbe::Function<'static>,
        expr: &LiteralExpr,
    ) -> EmitterResult<(qbe::Type<'static>, qbe::Value)> {
        let v = &expr.value;
        match v {
            LiteralValue::NumberFloat(v) => {
                let temp = self.new_temp();
                func.assign_instr(
                    temp.clone(),
                    qbe::Type::Double,
                    qbe::Instr::Copy(qbe::Value::Const(*v as u64)),
                );

                return Ok((qbe::Type::Double, temp));
            }
            LiteralValue::NumberInt(v) => {
                let temp = self.new_temp();
                func.assign_instr(
                    temp.clone(),
                    qbe::Type::Word,
                    qbe::Instr::Copy(qbe::Value::Const(*v as u64)),
                );

                return Ok((qbe::Type::Word, temp));
            }
            LiteralValue::String(_) => todo!(),
            LiteralValue::Boolean(_) => todo!(),
            LiteralValue::Null => todo!(),
        };
    }

    /// Creates a new temporary, returns the generated qbe::Value
    fn new_temp(&mut self) -> qbe::Value {
        self.tmp_counter += 1;
        qbe::Value::Temporary(format!("tmp.{}", self.tmp_counter))
    }
}
