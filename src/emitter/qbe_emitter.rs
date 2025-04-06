use crate::ast::LiteralValue;
use crate::ast::{BinExpr, BinOp, Expr, GroupExpr, LiteralExpr, Stmt, UnOp, UnaryExpr};
use crate::emitter::EmitterResult;
use qbe;

use super::Emitter;

/// Public struct for the QBE IR Emitter.
pub struct QBEEmitter<'a> {
    /// Counts total number of temporaries created.
    tmp_counter: usize,

    /// Stmts to compile.
    stmts: &'a Vec<Stmt>,

    /// Current module.
    /// This is usually 1 module per file basis.
    module: qbe::Module<'a>,
    // Contains all functions created in current module.
    // functions: Vec<qbe::Function<'static>>,

    // Contains all data definations created in current module.
    // data_defs: Vec<qbe::DataDef<'static>>,
}

/// Impl From for QBEEmitter.
impl<'a> From<&'a Vec<Stmt>> for QBEEmitter<'a> {
    fn from(stmts: &'a Vec<Stmt>) -> Self {
        Self {
            tmp_counter: 0,
            stmts,
            module: qbe::Module::new(),
        }
    }
}

/// Impl Emitter trait for QBEEmitter.
impl Emitter for QBEEmitter<'_> {
    fn emit_ir(&mut self) -> EmitterResult<String> {
        self.emit();
        Ok(format!("{}", self.module))
    }
}

/// More impl for QBEEmitter.
impl QBEEmitter<'_> {
    /// Top level emit function to start emitting.
    fn emit(&mut self) {
        let mut main_func = qbe::Function::new(
            qbe::Linkage::public(),
            "main",
            Vec::new(),
            Some(qbe::Type::Word),
        );
        main_func.add_block("start");
        for stmt in self.stmts.iter() {
            // self.emit_expr(&mut main_func, stmt).unwrap();
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
        main_func.add_instr(qbe::Instr::Ret(Some(qbe::Value::Const(0_u64))));
        self.module.add_function(main_func);
    }

    /// Emit generic expression ast.
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
            _ => todo!("implement"),
        }
    }

    /// Emit binary operation ast.
    fn emit_binary(
        &mut self,
        func: &mut qbe::Function<'static>,
        expr: &BinExpr,
    ) -> EmitterResult<(qbe::Type<'static>, qbe::Value)> {
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
                _ => todo!("TODO: other binary operations."),
            },
        );

        Ok((ty, temp))
    }

    /// Emit unary operation ast.
    fn emit_unary(
        &mut self,
        func: &mut qbe::Function<'static>,
        expr: &UnaryExpr,
    ) -> EmitterResult<(qbe::Type<'static>, qbe::Value)> {
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

    /// Emits grouping ast.
    fn emit_grouping(
        &mut self,
        func: &mut qbe::Function<'static>,
        expr: &GroupExpr,
    ) -> EmitterResult<(qbe::Type<'static>, qbe::Value)> {
        let temp = self.new_temp();
        let (_, value) = self.emit_expr(func, &expr.value)?;
        let ty = qbe::Type::Word;
        func.assign_instr(temp.clone(), ty.clone(), qbe::Instr::Copy(value));
        Ok((ty, temp))
    }

    /// Emits literal values in form of temporaries.
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

                Ok((qbe::Type::Double, temp))
            }
            LiteralValue::NumberInt(v) => {
                let temp = self.new_temp();
                func.assign_instr(
                    temp.clone(),
                    qbe::Type::Word,
                    qbe::Instr::Copy(qbe::Value::Const(*v as u64)),
                );

                Ok((qbe::Type::Word, temp))
            }
            LiteralValue::String(_) => todo!(),
            LiteralValue::Boolean(_) => todo!(),
            LiteralValue::Null => todo!(),
        }
    }

    /// Creates a new temporary, returns the generated qbe::Value
    fn new_temp(&mut self) -> qbe::Value {
        self.tmp_counter += 1;
        qbe::Value::Temporary(format!("tmp.{}", self.tmp_counter))
    }
}
