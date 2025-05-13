use anyhow::bail;

use crate::ast::{CallExpr, Expr, FnDecl, Stmt, StructDecl};

use super::{EvePass, EvePassImmutable, PassResult};

/// This pass checks for existence of all the non extern called function.
pub struct AllFnExistence {
    pub fn_decls: Vec<FnDecl>,
    pub st_decls: Vec<StructDecl>,
}

impl EvePass for AllFnExistence {
    fn new(fn_decls: Vec<FnDecl>, st_decls: Vec<StructDecl>) -> Self {
        Self { fn_decls, st_decls }
    }
}

impl EvePassImmutable for AllFnExistence {
    fn run_pass(&self) -> PassResult {
        let mut errs = vec![];
        for fns in &self.fn_decls {
            for stmt in &fns.body {
                if let Stmt::Expression(Expr::Call(call)) = stmt {
                    if let Err(err) = self.check_fn_existence(call) {
                        errs.push(err);
                    }
                }
            }
        }

        if !errs.is_empty() {
            return Err(errs);
        }

        Ok((self.fn_decls.to_owned(), self.st_decls.to_owned()))
    }
}

impl AllFnExistence {
    fn check_fn_existence(&self, call: &CallExpr) -> Result<(), anyhow::Error> {
        match &call.callee {
            Expr::Variable(var) => {
                if self.fn_decls.iter().any(|decl| decl.name == var.name) {
                    Ok(())
                } else {
                    bail!(
                        "Call to undefined function '{}', line {}",
                        var.name,
                        var.metadata.line
                    );
                }
            }
            _ => unreachable!(),
        }
    }
}
