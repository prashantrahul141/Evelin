use return_type::ReturnType;
use struct_field_missing_and_unknown::StructFieldMissingAndUnknown;
use struct_init_unique_fields::StructInitUniqueField;

use crate::ast::{FnDecl, StructDecl};

mod return_type;
mod struct_field_missing_and_unknown;
mod struct_init_unique_fields;

type PassResult = anyhow::Result<(Vec<FnDecl>, Vec<StructDecl>), Vec<anyhow::Error>>;

pub(super) trait EvePass {
    fn run_pass(&self, fn_decls: Vec<FnDecl>, st_decl: Vec<StructDecl>) -> PassResult;
}

pub fn run_passes(fn_: Vec<FnDecl>, st_: Vec<StructDecl>) -> PassResult {
    let p = StructInitUniqueField {};
    let (fn_, st_) = p.run_pass(fn_, st_)?;

    let p = StructFieldMissingAndUnknown {};
    let (fn_, st) = p.run_pass(fn_, st_)?;

    let p = ReturnType {};
    p.run_pass(fn_, st)
}
