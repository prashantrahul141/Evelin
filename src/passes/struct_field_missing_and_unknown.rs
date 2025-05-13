use std::collections::HashSet;

use anyhow::anyhow;

use crate::ast::{FnDecl, Stmt, StructDecl, StructInitStmt};

use super::{EvePass, EvePassImmutable, PassResult, PassResultGeneric};

/// This pass checks for missing and unknown fields in struct initilisation.
pub struct StructFieldMissingAndUnknown {
    fn_decls: Vec<FnDecl>,
    st_decls: Vec<StructDecl>,
}

impl EvePass for StructFieldMissingAndUnknown {
    fn new(fn_decls: Vec<FnDecl>, st_decls: Vec<StructDecl>) -> Self {
        Self { fn_decls, st_decls }
    }
}

impl EvePassImmutable for StructFieldMissingAndUnknown {
    fn run_pass(&self) -> PassResult {
        for fns in &self.fn_decls {
            for stmt in &fns.body {
                if let Stmt::StructInit(st) = stmt {
                    self.check_struct(st)?;
                }
            }
        }
        Ok((self.fn_decls.to_owned(), self.st_decls.to_owned()))
    }
}

impl StructFieldMissingAndUnknown {
    fn check_struct(&self, st_init: &StructInitStmt) -> PassResultGeneric<()> {
        let mut err = vec![];
        if let Some(decl) = self.st_decls.iter().find(|x| x.name == st_init.struct_name) {
            // Make sets from fields list
            // decl fields - init fields = missing fields
            // init fields - decl fields = unknown fields
            let decl_fields: HashSet<String> =
                decl.fields.iter().map(|x| x.field_name.clone()).collect();

            let init_fields: HashSet<String> = st_init
                .arguments
                .iter()
                .map(|x| x.field_name.clone())
                .collect();

            // decl_fields - init_fields
            for missing_field in decl_fields.difference(&init_fields) {
                err.push(anyhow!(format!(
                    "Field '{}' missing in struct '{}', line {}",
                    missing_field, &decl.name, decl.metadata.line
                )));
            }

            // init_fields - decl_fields
            for unknown_field in init_fields.difference(&decl_fields) {
                err.push(anyhow!(format!(
                    "Unknown field '{}' in struct '{}', line {}",
                    unknown_field, &decl.name, decl.metadata.line
                )));
            }
        }

        if !err.is_empty() {
            return Err(err);
        }

        Ok(())
    }
}
