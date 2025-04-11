use std::collections::HashMap;

use crate::ast::{
    BinExpr, BinOp, CallExpr, Expr, FieldAccessExpr, FnDecl, GroupExpr, LiteralExpr, LiteralValue,
    NativeCallExpr, Stmt, StructDecl, TokenType, UnOp, UnaryExpr, VariableExpr,
};
use crate::die;
use crate::emitter::EmitterResult;
use anyhow::bail;
use log::{debug, error};
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

    /// Scopes for variables.
    scopes: Vec<HashMap<String, (qbe::Type<'static>, qbe::Value)>>,

    /// Current module.
    /// This is usually 1 module per file basis.
    module: qbe::Module<'a>,

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
            scopes: vec![],
            module: qbe::Module::new(),
            emitted_data_defs: vec![],
        }
    }
}

/// Impl Emitter trait for QBEEmitter.
impl Emitter for QBEEmitter<'_> {
    fn emit_ir(&mut self) -> EmitterResult<String> {
        self.emit_data_defs();
        self.emit_functions();
        Ok(self.module.to_string())
    }
}

/// More impl for QBEEmitter.
impl QBEEmitter<'_> {
    /// Emits all parsed structs
    fn emit_data_defs(&mut self) {
        self.init_data_def();
        for struc in self.struct_decls {}
    }

    /// Emits a single function
    fn emit_data_body(&mut self) {}

    // Emits initialization data definition
    fn init_data_def(&mut self) {
        self.module.add_data(qbe::DataDef::new(
            qbe::Linkage::private(),
            "___FMT_INT",
            None,
            vec![
                (qbe::Type::Byte, qbe::DataItem::Str("%ld".into())),
                (qbe::Type::Byte, qbe::DataItem::Const(0)),
            ],
        ));

        self.module.add_data(qbe::DataDef::new(
            qbe::Linkage::private(),
            "___FMT_DOUBLE",
            None,
            vec![
                (qbe::Type::Byte, qbe::DataItem::Str("%lf".into())),
                (qbe::Type::Byte, qbe::DataItem::Const(0)),
            ],
        ));
    }

    /// Emits all parsed functions
    fn emit_functions(&mut self) -> EmitterResult<()> {
        for func in self.fn_decls {
            self.emit_function(func)?;
        }
        Ok(())
    }

    /// Emits a single function
    fn emit_function(&mut self, func: &FnDecl) -> EmitterResult<()> {
        self.scopes.push(HashMap::new());
        let mut func_block = qbe::Function::new(
            qbe::Linkage::public(),
            &func.name,
            func.parameter
                .iter()
                .map(|x| {
                    let ty = qbe::Type::try_from(x.1.clone())?;
                    let val = self.new_var(ty.clone(), x.0.clone())?;
                    Ok((ty, val))
                })
                .collect::<anyhow::Result<Vec<_>>>()?,
            qbe::Type::try_from(func.return_type.clone()).ok(),
        );
        func_block.add_block("start");
        self.emit_function_body(&mut func_block, &func.body);
        func_block.add_instr(qbe::Instr::Ret(None));
        self.module.add_function(func_block);
        self.scopes.pop();
        Ok(())
    }

    /// Emits a single function
    fn emit_function_body(&mut self, func: &mut qbe::Function<'static>, fn_body: &Vec<Stmt>) {
        for stmt in fn_body {
            let _ = self.emit_stmt(func, stmt);
        }
    }

    // Emits statement
    fn emit_stmt(&mut self, func: &mut qbe::Function<'static>, stmt: &Stmt) -> EmitterResult<()> {
        match stmt {
            Stmt::Block(blk) => self.emit_block(func, &blk.stmts),
            Stmt::Let(_) => todo!(),
            Stmt::StructInit(_) => todo!(),
            Stmt::If(stmt) => {
                self.emit_if_stmt(func, &stmt.condition, &stmt.if_branch, &stmt.else_branch)
            }
            Stmt::Print(expr) => self.emit_print_stmt(func, &expr.value),
            Stmt::Return(expr) => self.emit_return_stmt(func, &expr.value),
            Stmt::Expression(expr) => self.emit_expr_stmt(func, &expr),
        }
    }

    /// emits print statement based upon expression type.
    fn emit_print(
        &mut self,
        func: &mut qbe::Function<'static>,
        expr: &Expr,
    ) -> EmitterResult<(qbe::Type<'static>, qbe::Value)> {
        let (ty, value) = self.emit_expr(func, expr)?;

        let fmt = match ty {
            qbe::Type::Long => "___FMT_INT",
            qbe::Type::Double => "___FMT_DOUBLE",
            _ => todo!(),
        };

        func.add_instr(qbe::Instr::Call(
            "printf".into(),
            vec![
                (qbe::Type::Long, qbe::Value::Global(fmt.into())),
                (ty.clone(), value.clone()),
            ],
            Some(1),
        ));

        Ok((ty, value))
    }

    /// emits return statement
    fn emit_return(
        &mut self,
        func: &mut qbe::Function<'static>,
        expr: &Expr,
    ) -> EmitterResult<(qbe::Type<'static>, qbe::Value)> {
        let (ty, value) = self.emit_expr(func, expr)?;
        func.add_instr(qbe::Instr::Ret(Some(value.clone())));
        Ok((ty, value))
    }

    /// Emit generic expression ast.
    fn emit_expr(
        &mut self,
        func: &mut qbe::Function<'static>,
        expr: &Expr,
    ) -> EmitterResult<(qbe::Type<'static>, qbe::Value)> {
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
        let (ty_left, left) = self.emit_expr(func, &expr.left)?;
        let (ty_right, right) = self.emit_expr(func, &expr.right)?;
        let tmp = self.new_tmp();

        let mut ty = qbe::Type::Long;
        if matches!(ty_left, qbe::Type::Double) || matches!(ty_right, qbe::Type::Double) {
            ty = qbe::Type::Double;
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
        let arg = if let Some(arg_expr) = &call.arg {
            vec![self.emit_expr(func, arg_expr)?]
        } else {
            vec![]
        };

        let tmp = self.new_tmp();

        if let Expr::Variable(var) = &call.callee {
            func.assign_instr(
                tmp.clone(),
                qbe::Type::Long,
                qbe::Instr::Call(var.name.clone(), arg, None),
            );
        } else {
            error!("Expected function name got '{:?}' instead", call.callee);
            bail!("Expected function name got '{:?}' instead", call.callee);
        }

        Ok((qbe::Type::Long, tmp))
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
        let ty = qbe::Type::Long;
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
        let (ty, tmp) = self.get_var(&expr.name)?.clone();
        Ok((ty, tmp))
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
                let tmp = self.new_tmp();
                let ty = qbe::Type::Double;
                func.assign_instr(
                    tmp.clone(),
                    ty.clone(),
                    qbe::Instr::Copy(qbe::Value::Const(*v as u64)),
                );

                Ok((ty, tmp))
            }
            LiteralValue::NumberInt(v) => {
                let tmp = self.new_tmp();
                let ty = qbe::Type::Long;
                func.assign_instr(
                    tmp.clone(),
                    ty.clone(),
                    qbe::Instr::Copy(qbe::Value::Const(*v as u64)),
                );

                Ok((ty, tmp))
            }
            LiteralValue::String(_) => todo!(),
            LiteralValue::Boolean(_) => todo!(),
            LiteralValue::Null => todo!(),
        }
    }

    /// Creates a new bindings bound to current scope.
    fn new_var(&mut self, ty: qbe::Type<'static>, name: String) -> EmitterResult<qbe::Value> {
        if self.get_var(&name).is_ok() {
            bail!("Re-declaration of variable : {}", name);
        }

        let tmp = self.new_tmp();
        let scope = self
            .scopes
            .last_mut()
            .expect("Expected last scope to be present");
        scope.insert(name.into(), (ty.to_owned(), tmp.to_owned()));

        Ok(tmp)
    }

    /// Retrieves an existing bind
    /// Searches in reverse order of scopes.
    fn get_var(&mut self, name: &String) -> EmitterResult<&(qbe::Type<'static>, qbe::Value)> {
        self.scopes
            .iter()
            .rev()
            .filter_map(|x| x.get(name))
            .next()
            .ok_or_else(|| anyhow!("undefined variable: {}", name))
    }

    /// Creates a new temporary, returns the generated qbe::Value
    fn new_tmp(&mut self) -> qbe::Value {
        self.tmp_counter += 1;
        trace!("creating new tmp = %tmp.{}", self.tmp_counter);
        qbe::Value::Temporary(format!("tmp.{}", self.tmp_counter))
    }

    /// Creates and returns a new temporary from a given name,
    fn new_tmp_from(&mut self, name: &String) -> qbe::Value {
        trace!("creating new tmp = %tmp.{}", name);
        qbe::Value::Temporary(format!("tmp.{}", name))
    }
}

impl TryFrom<TokenType> for qbe::Type<'_> {
    type Error = anyhow::Error;

    fn try_from(value: TokenType) -> Result<Self, Self::Error> {
        match value {
            TokenType::TypeI64 => Ok(qbe::Type::Long),
            TokenType::TypeF64 => Ok(qbe::Type::Double),
            TokenType::TypeVoid => Err(anyhow!("qbe::Type::TryFrom recieved type = TypeVoid")),
            v => {
                die!("qbe::Value::from failed, recieved token type: {}", v);
            }
        }
    }
}
