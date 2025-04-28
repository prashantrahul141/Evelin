use all_fn_existence::AllFnExistence;
use main_fn_existence::MainFnExistence;
use struct_field_missing_and_unknown::StructFieldMissingAndUnknown;
use struct_init_unique_fields::StructInitUniqueField;

use crate::ast::{FnDecl, StructDecl};

mod all_fn_existence;
mod main_fn_existence;
mod struct_field_missing_and_unknown;
mod struct_init_unique_fields;

type PassResultGeneric<T> = anyhow::Result<T, Vec<anyhow::Error>>;
type PassResult = PassResultGeneric<(Vec<FnDecl>, Vec<StructDecl>)>;

pub(super) trait EvePass {
    fn run_pass(&self, fn_decls: Vec<FnDecl>, st_decl: Vec<StructDecl>) -> PassResult;
}

pub fn run_passes(fn_: Vec<FnDecl>, st_: Vec<StructDecl>) -> PassResult {
    let p = MainFnExistence {};
    let (fn_, st_) = p.run_pass(fn_, st_)?;

    let p = AllFnExistence {};
    let (fn_, st_) = p.run_pass(fn_, st_)?;

    let p = StructInitUniqueField {};
    let (fn_, st_) = p.run_pass(fn_, st_)?;

    let p = StructFieldMissingAndUnknown {};
    p.run_pass(fn_, st_)
}
