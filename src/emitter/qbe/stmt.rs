use std::collections::HashMap;

use crate::ast::{Expr, Stmt};
use crate::die;
use crate::emitter::EmitterResult;
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
            qbe::Type::Word => "___FMT_INT",
            qbe::Type::Single => "___FMT_SINGLE",
            qbe::Type::Long => "___FMT_POINTER",
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
}
