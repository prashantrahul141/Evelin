use crate::{
    ast::{FnDecl, Stmt, StructDecl},
    utils::{MessageType, WarningType, report_message},
};

use super::{EvePass, PassResult};

/// Removes code which is sure to be never be executed.
/// This pass modifies the ast.
pub struct DeadCodeElimination {}

impl EvePass for DeadCodeElimination {
    fn run_pass(&self, mut fn_decls: Vec<FnDecl>, st_decl: Vec<StructDecl>) -> PassResult {
        for fns in fn_decls.iter_mut().filter(|x| x.name != "main") {
            self.remove_stmt_after_return(fns);
        }

        Ok((fn_decls, st_decl))
    }
}

impl DeadCodeElimination {
    fn remove_stmt_after_return(&self, fns: &mut FnDecl) {
        let index = fns.body.iter().position(|x| matches!(x, &Stmt::Return(_)));

        if let Some(i) = index.filter(|&i| i != fns.body.len() - 1) {
            report_message(
                format!(
                    "Code after return statement in function '{}' will be ignored",
                    &fns.name
                ),
                MessageType::Warning(WarningType::None),
            );
            fns.body.truncate(i);
        }
    }
}
