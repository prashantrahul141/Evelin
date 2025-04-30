use anyhow::bail;

use crate::ast::{FnDecl, Stmt, StructDecl, StructInitStmt};

use super::{EvePass, EvePassImmutable};

/// This passes checks that all fields specified in struct initialization are unique.
pub struct StructInitUniqueField {
    fn_decls: Vec<FnDecl>,
    st_decls: Vec<StructDecl>,
}

impl EvePass for StructInitUniqueField {
    fn new(fn_decls: Vec<FnDecl>, st_decls: Vec<StructDecl>) -> Self {
        Self { fn_decls, st_decls }
    }
}

impl EvePassImmutable for StructInitUniqueField {
    fn run_pass(&self) -> anyhow::Result<(Vec<FnDecl>, Vec<StructDecl>), Vec<anyhow::Error>> {
        let mut err = vec![];
        for fns in &self.fn_decls {
            for stmt in &fns.body {
                if let Stmt::StructInit(st) = stmt {
                    if let Err(e) = self.check_struct(st) {
                        err.push(e);
                    }
                }
            }
        }

        if !err.is_empty() {
            return Err(err);
        }

        Ok((self.fn_decls.to_owned(), self.st_decls.to_owned()))
    }
}

impl StructInitUniqueField {
    fn check_struct(&self, st_init: &StructInitStmt) -> anyhow::Result<()> {
        let mut m = vec![];
        for i in &st_init.arguments {
            if m.contains(&i.field_name) {
                bail!(
                    "field '{}' is already defined for '{}' of type '{}'",
                    &i.field_name,
                    &st_init.name,
                    &st_init.struct_name
                );
            }

            m.push(i.field_name.clone())
        }
        Ok(())
    }
}
