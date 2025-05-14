use std::collections::HashSet;

use anyhow::{anyhow, bail};

use crate::ast::{
    BlockStmt, BreakStmt, DType, EveTypes, IfStmt, LetStmt, LoopStmt, PrintStmt, ReturnStmt,
    StructInitStmt,
};

use super::TypeSystem;

impl TypeSystem<'_> {
    pub(super) fn check_block(&mut self, block: &mut BlockStmt) -> anyhow::Result<DType> {
        for stmt in &mut block.stmts {
            self.check_stmt(stmt)?;
        }
        Ok(DType::Primitive(EveTypes::Void))
    }

    pub(super) fn check_let(&mut self, le: &mut LetStmt) -> anyhow::Result<DType> {
        let ty = self.check_expr(&mut le.initialiser)?;
        self.def_env(le.name.clone(), ty.clone());
        Ok(ty)
    }

    pub(super) fn check_stinit(&mut self, st_init: &mut StructInitStmt) -> anyhow::Result<DType> {
        let decl = self
            .st_decls
            .iter()
            .find(|x| x.name == st_init.struct_name)
            .ok_or(anyhow!(
                "Struct '{}' not defined at, line {}",
                &st_init.struct_name,
                st_init.metadata.line
            ))?;

        let decl_fields: HashSet<(String, DType)> = decl
            .fields
            .iter()
            .map(|x| (x.field_name.clone(), x.field_type.clone()))
            .collect();

        let init_fields: HashSet<(String, DType)> = st_init
            .arguments
            .iter_mut()
            .map(|x| {
                let expr_ty = self.check_expr(&mut x.field_expr)?;
                Ok::<(String, DType), anyhow::Error>((x.field_name.clone(), expr_ty))
            })
            .collect::<Result<_, _>>()?;

        // decl_fields - init_fields
        if let Some(missing_field) = decl_fields.difference(&init_fields).next() {
            bail!(
                "Invalid type for '{}' in struct '{}', line {}",
                missing_field.0,
                &decl.name,
                decl.metadata.line
            );
        }

        // f.field_name
        self.def_env(
            st_init.name.clone(),
            DType::Derived(st_init.struct_name.clone()),
        );
        Ok(DType::Primitive(EveTypes::Void))
    }

    pub(super) fn check_if(&mut self, ifst: &mut IfStmt) -> anyhow::Result<DType> {
        let _ = self.check_expr(&mut ifst.condition)?;
        let _ = self.check_stmt(&mut ifst.if_branch)?;
        if let Some(else_branch) = &mut ifst.else_branch {
            self.check_stmt(else_branch)?;
        }
        Ok(DType::Primitive(EveTypes::Void))
    }

    pub(super) fn check_loop(&mut self, loop_stmt: &mut LoopStmt) -> anyhow::Result<DType> {
        self.check_stmt(&mut loop_stmt.body)
    }

    pub(super) fn check_break(&mut self, _p: &mut BreakStmt) -> anyhow::Result<DType> {
        Ok(DType::Primitive(EveTypes::Void))
    }

    pub(super) fn check_print(&mut self, p: &mut PrintStmt) -> anyhow::Result<DType> {
        self.check_expr(&mut p.value)
    }

    pub(super) fn check_return(&mut self, ret: &mut ReturnStmt) -> anyhow::Result<DType> {
        match &mut ret.value {
            Some(val) => self.check_expr(val),
            None => Ok(DType::Primitive(EveTypes::Void)),
        }
    }
}
