use std::collections::HashMap;

use crate::ast::{Expr, IfStmt, LetStmt, PrintStmt, ReturnStmt, Stmt, StructInitStmt};
use crate::die;
use crate::emitter::EmitterResult;
use anyhow::{Context, bail};
use log::{error, trace};
use qbe;

use super::QBEEmitter;

impl QBEEmitter<'_> {
    // Emits statement
    pub(super) fn emit_stmt(
        &mut self,
        func: &mut qbe::Function<'static>,
        stmt: &Stmt,
    ) -> EmitterResult<()> {
        trace!("emitting new stmt");
        match stmt {
            Stmt::Block(stmt) => self.emit_block(func, &stmt.stmts),
            Stmt::Let(stmt) => self.emit_let(func, stmt),
            Stmt::StructInit(stmt) => self.emit_struct_init(func, stmt),
            Stmt::If(stmt) => self.emit_if_stmt(func, stmt),
            Stmt::Print(stmt) => self.emit_print_stmt(func, stmt),
            Stmt::Return(stmt) => self.emit_return_stmt(func, stmt),
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
    fn emit_let(&mut self, func: &mut qbe::Function<'static>, le: &LetStmt) -> EmitterResult<()> {
        let (ty, value) = self.emit_expr(func, &le.initialiser)?;
        let result_value = self.new_var(ty.clone(), le.name.clone())?;
        func.assign_instr(result_value, ty, qbe::Instr::Copy(value));
        Ok(())
    }

    // emits struct initialization
    fn emit_struct_init(
        &mut self,
        func: &mut qbe::Function<'static>,
        st_init: &StructInitStmt,
    ) -> EmitterResult<()> {
        trace!("emitting struct init stmt");

        let (meta, size) = self
            .struct_meta
            .get(&st_init.struct_name)
            .with_context(|| {
                format!(
                    "Initialiser of undeclared struct '{}', line {}",
                    &st_init.struct_name, st_init.metadata.line
                )
            })?
            .to_owned();

        let type_def = self
            .type_defs
            .iter()
            .find(|x| x.name == st_init.struct_name)
            .cloned()
            .with_context(|| {
                format!(
                    "Initialiser of undeclared struct '{}', line {}",
                    st_init.struct_name, st_init.metadata.line
                )
            })?;

        let boxed_type_def = Box::new(type_def);
        let tmp = self.new_var(
            qbe::Type::Aggregate(Box::leak(boxed_type_def)),
            st_init.name.to_owned(),
        )?;
        func.assign_instr(tmp.clone(), qbe::Type::Long, qbe::Instr::Alloc8(size));

        for arg in &st_init.arguments {
            // get meta about arg
            let (field_type, offset) = meta.get(&arg.field_name).with_context(|| {
                format!(
                    "Unknown field : '{}', line {}",
                    arg.field_name, arg.metadata.line
                )
            })?;

            let (_, expr_tmp) = self.emit_expr(func, &arg.field_expr)?;
            match field_type {
                qbe::Type::Aggregate(_) => {
                    bail!(
                        "Aggregate types inside structs are not supported yet, line {}",
                        arg.metadata.line
                    );
                }
                _ => {
                    let field_tmp = self.new_tmp();
                    func.assign_instr(
                        field_tmp.clone(),
                        qbe::Type::Long,
                        qbe::Instr::Add(tmp.clone(), qbe::Value::Const(*offset)),
                    );

                    func.add_instr(qbe::Instr::Store(field_type.clone(), field_tmp, expr_tmp));
                }
            }
        }

        Ok(())
    }

    /// Emits if statement
    fn emit_if_stmt(
        &mut self,
        func: &mut qbe::Function<'static>,
        if_stmt: &IfStmt,
    ) -> EmitterResult<()> {
        trace!("emitting if stmt");
        let (_, cond_result) = self.emit_expr(func, &if_stmt.condition)?;
        self.tmp_counter += 1;

        let if_label = format!("cond.{}.if", self.tmp_counter);
        let else_label = format!("cond.{}.else", self.tmp_counter);
        let end_label = format!("cond.{}.end", self.tmp_counter);

        func.add_instr(qbe::Instr::Jnz(
            cond_result,
            if_label.clone(),
            if if_stmt.else_branch.is_some() {
                else_label.clone()
            } else {
                end_label.clone()
            },
        ));

        func.add_block(if_label);
        self.emit_stmt(func, &if_stmt.if_branch)?;

        if let Some(else_branch) = &if_stmt.else_branch {
            trace!("emitting else clause for if stmt");
            // avoid fallthrough into else block even after executing if block.
            if !func.blocks.last().is_some_and(|b| b.jumps()) {
                func.add_instr(qbe::Instr::Jmp(end_label.clone()));
            }

            func.add_block(else_label);
            self.emit_stmt(func, else_branch)?;
        }

        func.add_block(end_label);

        Ok(())
    }

    /// emits print statement based upon expression type.
    fn emit_print_stmt(
        &mut self,
        func: &mut qbe::Function<'static>,
        print_stmt: &PrintStmt,
    ) -> EmitterResult<()> {
        trace!("emitting print stmt expr = {:?}", print_stmt.value);
        let (ty, value) = self.emit_expr(func, &print_stmt.value)?;

        let fmt = match ty {
            qbe::Type::Word => "___FMT_WORD",
            qbe::Type::Long => "___FMT_LONG",
            qbe::Type::Single => "___FMT_SINGLE",
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
        return_stmt: &ReturnStmt,
    ) -> EmitterResult<()> {
        trace!("emitting new return stmt expr = {:?}", &return_stmt.value);
        let mut inst = qbe::Instr::Ret(None);
        if let Some(expr) = &return_stmt.value {
            let (_, value) = self.emit_expr(func, expr)?;
            inst = qbe::Instr::Ret(Some(value));
        }
        func.add_instr(inst);
        Ok(())
    }

    /// Emit expression stmt
    fn emit_expr_stmt(
        &mut self,
        func: &mut qbe::Function<'static>,
        expr: &Expr,
    ) -> EmitterResult<()> {
        self.emit_expr(func, expr)?;
        Ok(())
    }
}
