use crate::{
    ast::{FnDecl, Stmt, StructDecl},
    utils::{MessageType, report_message},
};

use super::{EvePass, PassResult};

pub struct DeadCodeElimination {}

impl EvePass for DeadCodeElimination {
    fn run_pass(&self, mut fn_decls: Vec<FnDecl>, st_decl: Vec<StructDecl>) -> PassResult {
        for fns in &mut fn_decls {
            self.remove_stmt_after_return(fns);
        }

        Ok((fn_decls, st_decl))
    }
}

impl DeadCodeElimination {
    fn remove_stmt_after_return(&self, fns: &mut FnDecl) {
        let mut index = fns.body.len() - 1;
        for (i, stmt) in fns.body.iter().enumerate() {
            if matches!(stmt, Stmt::Return(_)) {
                index = i;
            }
        }

        if index != fns.body.len() - 1 {
            report_message(
                format!(
                    "Code after return statement in function '{}' will be ignored",
                    &fns.name
                ),
                MessageType::Warning,
            );
            fns.body.truncate(index);
        }
    }
}
