use struct_field_check::StructFieldCheck;
use struct_init_unique_fields::StructInitUniqueField;
use type_check::TypeChecker;

use crate::ast::{FnDecl, StructDecl};

mod struct_field_check;
mod struct_init_unique_fields;
mod type_check;

type PassResult = anyhow::Result<(Vec<FnDecl>, Vec<StructDecl>), Vec<anyhow::Error>>;

pub(super) trait EvePass {
    fn run_pass(&self, fn_decls: Vec<FnDecl>, st_decl: Vec<StructDecl>) -> PassResult;
}

pub fn run_passes(fn_: Vec<FnDecl>, st_: Vec<StructDecl>) -> PassResult {
    let p = StructInitUniqueField {};
    let (fn_, st_) = p.run_pass(fn_, st_)?;

    let p = TypeChecker {};
    let (fn_, st) = p.run_pass(fn_, st_)?;

    let p = StructFieldCheck {};
    p.run_pass(fn_, st)
}
