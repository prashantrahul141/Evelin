use std::collections::HashSet;

use anyhow::anyhow;

use crate::ast::{FnDecl, Stmt, StructDecl, StructInitStmt};

use super::{EvePass, PassResult, PassResultGeneric};

/// This pass checks for missing and unknown fields in struct initilisation.
pub struct StructFieldMissingAndUnknown {}

impl EvePass for StructFieldMissingAndUnknown {
    fn run_pass(&self, fn_decls: Vec<FnDecl>, st_decl: Vec<StructDecl>) -> PassResult {
        for fns in &fn_decls {
            for stmt in &fns.body {
                if let Stmt::StructInit(st) = stmt {
                    self.check_struct(&st_decl, st)?;
                }
            }
        }
        Ok((fn_decls, st_decl))
    }
}

impl StructFieldMissingAndUnknown {
    fn check_struct(
        &self,
        st_decl: &[StructDecl],
        st_init: &StructInitStmt,
    ) -> PassResultGeneric<()> {
        let mut err = vec![];
        if let Some(decl) = st_decl.iter().find(|x| x.name == st_init.struct_name) {
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
                    "Field '{}' missing in struct '{}'",
                    missing_field, &decl.name
                )));
            }

            // init_fields - decl_fields
            for unknown_field in init_fields.difference(&decl_fields) {
                err.push(anyhow!(format!(
                    "Unknown field '{}' in struct '{}'",
                    unknown_field, &decl.name
                )));
            }
        }

        if !err.is_empty() {
            return Err(err);
        }

        Ok(())
    }
}
