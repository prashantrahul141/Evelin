use crate::{
    ast::{FnDecl, Stmt, StructDecl},
    utils::{MessageType, WarningType, report_message},
};

use super::{EvePass, EvePassMutable, PassResult};

/// Removes code which is sure to be never be executed.
/// This pass modifies the ast.
pub struct DeadCodeElimination {
    pub fn_decls: Vec<FnDecl>,
    pub st_decls: Vec<StructDecl>,
}

impl EvePass for DeadCodeElimination {
    fn new(fn_decls: Vec<FnDecl>, st_decls: Vec<StructDecl>) -> Self {
        Self { fn_decls, st_decls }
    }
}

impl EvePassMutable for DeadCodeElimination {
    fn run_pass(&mut self) -> PassResult {
        for fns in self.fn_decls.iter_mut().filter(|x| x.name != "main") {
            Self::remove_stmt_after_return(fns);
        }

        Ok((self.fn_decls.to_owned(), self.st_decls.to_owned()))
    }
}

impl DeadCodeElimination {
    fn remove_stmt_after_return(fns: &mut FnDecl) {
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
