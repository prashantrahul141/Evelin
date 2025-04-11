mod utils;

use std::collections::HashMap;

use crate::ast::{
    BinExpr, BinOp, CallExpr, Expr, FieldAccessExpr, FnDecl, GroupExpr, LiteralExpr, LiteralValue,
    NativeCallExpr, Stmt, StructDecl, UnOp, UnaryExpr, VariableExpr,
};
use crate::die;
use crate::emitter::EmitterResult;
use anyhow::bail;
use log::{debug, error, info, trace};
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
        info!("creating new QBEEmitter instance");
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
        debug!("start emitting qbe ir");
        self.emit_data_defs();
        self.emit_functions()?;
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
        debug!("emiting initial data definition");
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
        debug!("Emitting functions");
        for func in self.fn_decls {
            self.emit_function(func)?;
        }
        Ok(())
    }

    /// Emits a single function
    fn emit_function(&mut self, func: &FnDecl) -> EmitterResult<()> {
        trace!("Emitting a new function: '{}'", &func.name);
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
        trace!("adding new function = {}", &func_block);
        self.module.add_function(func_block);
        self.scopes.pop();
        Ok(())
    }

    /// Emits a single function
    fn emit_function_body(&mut self, func: &mut qbe::Function<'static>, fn_body: &Vec<Stmt>) {
        trace!("emitting function body");
        for stmt in fn_body {
            let _ = self.emit_stmt(func, stmt);
        }
    }

    // Emits statement
    fn emit_stmt(&mut self, func: &mut qbe::Function<'static>, stmt: &Stmt) -> EmitterResult<()> {
        trace!("emitting new stmt");
        match stmt {
            Stmt::Block(blk) => self.emit_block(func, &blk.stmts),
            Stmt::Let(lt) => self.emit_let(func, &lt.name, &lt.initialiser),
            Stmt::StructInit(_) => todo!(),
            Stmt::If(stmt) => {
                self.emit_if_stmt(func, &stmt.condition, &stmt.if_branch, &stmt.else_branch)
            }
            Stmt::Print(expr) => self.emit_print_stmt(func, &expr.value),
            Stmt::Return(expr) => self.emit_return_stmt(func, &expr.value),
            Stmt::Expression(expr) => self.emit_expr_stmt(func, expr),
        }
    }

    /// Emits blocks
    fn emit_block(
        &mut self,
        func: &mut qbe::Function<'static>,
        stmts: &[Stmt],
    ) -> EmitterResult<()> {
        trace!("emitting new block");
        self.scopes.push(HashMap::new());
        for stmt in stmts.iter() {
            self.emit_stmt(func, stmt)?;
        }
        self.scopes.pop();
        Ok(())
    }

    // emits let declaration
    fn emit_let(
        &mut self,
        func: &mut qbe::Function<'static>,
        name: &str,
        init: &Expr,
    ) -> EmitterResult<()> {
        let (ty, value) = self.emit_expr(func, init)?;
        let result_value = self.new_var(ty.clone(), name.to_owned())?;
        func.assign_instr(result_value, ty, qbe::Instr::Copy(value));
        Ok(())
    }

    /// Emits if statement
    fn emit_if_stmt(
        &mut self,
        func: &mut qbe::Function<'static>,
        cond: &Expr,
        if_clause: &Stmt,
        else_clause: &Option<Stmt>,
    ) -> EmitterResult<()> {
        trace!("emitting if stmt");
        let (_, cond_result) = self.emit_expr(func, cond)?;
        self.tmp_counter += 1;

        let if_label = format!("cond.{}.if", self.tmp_counter);
        let else_label = format!("cond.{}.else", self.tmp_counter);
        let end_label = format!("cond.{}.end", self.tmp_counter);

        func.add_instr(qbe::Instr::Jnz(
            cond_result,
            if_label.clone(),
            if else_clause.is_some() {
                else_label.clone()
            } else {
                end_label.clone()
            },
        ));

        func.add_block(if_label);
        self.emit_stmt(func, if_clause)?;

        if let Some(else_clause) = else_clause {
            trace!("emitting else clause for if stmt");
            // avoid fallthrough into else block even after executing if block.
            if !func.blocks.last().is_some_and(|b| b.jumps()) {
                func.add_instr(qbe::Instr::Jmp(end_label.clone()));
            }

            func.add_block(else_label);
            self.emit_stmt(func, else_clause)?;
        }

        func.add_block(end_label);

        Ok(())
    }

    /// emits print statement based upon expression type.
    fn emit_print_stmt(
        &mut self,
        func: &mut qbe::Function<'static>,
        expr: &Expr,
    ) -> EmitterResult<()> {
        trace!("emitting print stmt expr = {:?}", expr);
        let (ty, value) = self.emit_expr(func, expr)?;

        let fmt = match ty {
            qbe::Type::Long => "___FMT_INT",
            qbe::Type::Double => "___FMT_DOUBLE",
            _ => {
                die!("formatting for this type doesn't exist");
            }
        };
        trace!("print FMT = {}", fmt);

        func.add_instr(qbe::Instr::Call(
            "printf".into(),
            vec![
                (qbe::Type::Long, qbe::Value::Global(fmt.into())),
                (ty.clone(), value.clone()),
            ],
            Some(1),
        ));

        Ok(())
    }

    /// emits return statement
    fn emit_return_stmt(
        &mut self,
        func: &mut qbe::Function<'static>,
        expr: &Expr,
    ) -> EmitterResult<()> {
        trace!("emitting new return stmt expr = {:?}", &expr);
        let (_, value) = self.emit_expr(func, expr)?;
        func.add_instr(qbe::Instr::Ret(Some(value.clone())));
        Ok(())
    }

    /// Emit expression stmt
    fn emit_expr_stmt(
        &mut self,
        func: &mut qbe::Function<'static>,
        expr: &Expr,
    ) -> EmitterResult<()> {
        let _ = self.emit_expr(func, expr);
        Ok(())
    }

    /// Emit expr
    fn emit_expr(
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
        trace!("emitting call expr call = {:?}", call);
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
        trace!("emitting native call expr call = {:?}", call);
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
}
