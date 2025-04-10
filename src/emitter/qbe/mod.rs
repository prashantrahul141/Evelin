use crate::ast::{BinExpr, BinOp, Expr, GroupExpr, LiteralExpr, Stmt, StructDecl, UnOp, UnaryExpr};
use crate::ast::{FnDecl, LiteralValue};
use crate::emitter::EmitterResult;
use qbe;

use super::Emitter;

/// Public struct for the QBE IR Emitter.
pub struct QBEEmitter<'a> {
    /// Counts total number of temporaries created.
    tmp_counter: usize,

    /// Function declarations
    fn_decls: &'a Vec<FnDecl>,

    /// Struct declarations
    struct_decls: &'a Vec<StructDecl>,

    /// Current module.
    /// This is usually 1 module per file basis.
    module: qbe::Module<'a>,

    /// Contains all functions created in current module.
    emitted_functions: Vec<qbe::Function<'static>>,

    // Contains all data definations created in current module.
    emitted_data_defs: Vec<qbe::DataDef<'static>>,
}

/// Impl From for QBEEmitter.
impl<'a> From<(&'a Vec<FnDecl>, &'a Vec<StructDecl>)> for QBEEmitter<'a> {
    fn from(decls: (&'a Vec<FnDecl>, &'a Vec<StructDecl>)) -> Self {
        Self {
            tmp_counter: 0,
            fn_decls: decls.0,
            struct_decls: decls.1,
            module: qbe::Module::new(),
            emitted_functions: vec![],
            emitted_data_defs: vec![],
        }
    }
}

/// Impl Emitter trait for QBEEmitter.
impl Emitter for QBEEmitter<'_> {
    fn emit_ir(&mut self) -> EmitterResult<String> {
        self.emit_data_defs();
        self.emit_functions();
        Ok(format!("{}", self.module))
    }
}

/// More impl for QBEEmitter.
impl QBEEmitter<'_> {
    /// Emits all parsed structs
    fn emit_data_defs(&mut self) {
        for struc in self.struct_decls {}
    }

    /// Emits a single function
    fn emit_data_def(&mut self) {}

    /// Emits all parsed functions
    fn emit_functions(&mut self) {
        for func in self.fn_decls {
            let mut func_block = qbe::Function::new(
                qbe::Linkage::public(),
                &func.name,
                Vec::new(),
                Some(qbe::Type::Word),
            );
            func_block.add_block("start");
            self.emit_function(&mut func_block, func);
            self.module.add_function(func_block);
        }
    }

    /// Emits a single function
    fn emit_function(&mut self, func_block: &mut qbe::Function<'static>, func_node: &FnDecl) {}

    /// Top level emit function to start emitting.
    fn emit(&mut self) {
        let mut main_func = qbe::Function::new(
            qbe::Linkage::public(),
            "main",
            Vec::new(),
            Some(qbe::Type::Word),
        );
        main_func.add_block("start");
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
                BinOp::Add => qbe::Instr::Add(left, right),
                BinOp::Sub => qbe::Instr::Sub(left, right),
                BinOp::Mul => qbe::Instr::Mul(left, right),
                BinOp::Div => qbe::Instr::Div(left, right),
                BinOp::Mod => qbe::Instr::Rem(left, right),
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
impl<'a> TryFrom<TokenType> for qbe::Type<'a> {
    type Error = ();

    fn try_from(value: TokenType) -> Result<Self, Self::Error> {
        match value {
            TokenType::TypeI64 => Ok(qbe::Type::Long),
            TokenType::TypeF64 => Ok(qbe::Type::Double),
            TokenType::TypeVoid => Err(()),
            v => {
                die!("qbe::Value::from failed, recieved token type: {}", v);
            }
        }
    }
}
