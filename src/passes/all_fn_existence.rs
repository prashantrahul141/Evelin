use anyhow::anyhow;

use crate::ast::{CallExpr, Expr, FnDecl, Stmt, StructDecl};

use super::{EvePass, PassResult, PassResultGeneric};

/// This pass checks for existence of all the non extern called function.
pub struct AllFnExistence {}

impl EvePass for AllFnExistence {
    fn run_pass(&self, fn_decls: Vec<FnDecl>, st_decl: Vec<StructDecl>) -> PassResult {
        for fns in &fn_decls {
            for stmt in &fns.body {
                if let Stmt::Expression(Expr::Call(call)) = stmt {
                    self.check_fn_existence(&fn_decls, call)?;
                }
            }
        }

        Ok((fn_decls, st_decl))
    }
}

impl AllFnExistence {
    fn check_fn_existence(&self, fn_decls: &[FnDecl], call: &CallExpr) -> PassResultGeneric<()> {
        let mut err = vec![];
        match &call.callee {
            Expr::Variable(var) => {
                if fn_decls.iter().any(|decl| decl.name == var.name) {
                    return Ok(());
                } else {
                    err.push(anyhow!("Call to undefined function '{}'", var.name));
                }
            }
            _ => unreachable!(),
        };
        Err(err)
    }
}
