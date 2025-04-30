use anyhow::anyhow;

use crate::ast::{FnDecl, StructDecl};

use super::{EvePass, EvePassImmutable, PassResult};

/// This checks for the existence of main function.
pub struct MainFnExistence {
    fn_decls: Vec<FnDecl>,
    st_decls: Vec<StructDecl>,
}

impl EvePass for MainFnExistence {
    fn new(fn_decls: Vec<FnDecl>, st_decls: Vec<StructDecl>) -> Self {
        Self { fn_decls, st_decls }
    }
}

impl EvePassImmutable for MainFnExistence {
    fn run_pass(&self) -> PassResult {
        if self.fn_decls.iter().any(|fns| fns.name == "main") {
            return Ok((self.fn_decls.to_owned(), self.st_decls.to_owned()));
        }

        Err(vec![anyhow!("No main function found.")])
    }
}
