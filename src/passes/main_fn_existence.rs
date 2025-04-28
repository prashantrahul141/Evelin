use anyhow::anyhow;

use crate::ast::{FnDecl, StructDecl};

use super::{EvePass, PassResult};

/// This checks for the existence of main function.
pub struct MainFnExistence {}

impl EvePass for MainFnExistence {
    fn run_pass(&self, fn_decls: Vec<FnDecl>, st_decl: Vec<StructDecl>) -> PassResult {
        if fn_decls.iter().any(|fns| fns.name == "main") {
            return Ok((fn_decls, st_decl));
        }

        Err(vec![anyhow!("No main function found.")])
    }
}
