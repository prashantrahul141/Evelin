use all_fn_existence::AllFnExistence;
use dead_code_elimination::DeadCodeElimination;
use main_fn_existence::MainFnExistence;
use struct_field_missing_and_unknown::StructFieldMissingAndUnknown;
use struct_init_unique_fields::StructInitUniqueField;
use type_check::TypeCheck;

use crate::ast::{FnDecl, StructDecl};

mod all_fn_existence;
mod dead_code_elimination;
mod main_fn_existence;
mod struct_field_missing_and_unknown;
mod struct_init_unique_fields;
mod type_check;

type PassResultGeneric<T> = anyhow::Result<T, Vec<anyhow::Error>>;
type PassResult = PassResultGeneric<(Vec<FnDecl>, Vec<StructDecl>)>;

pub(super) trait EvePass {
    fn new(fn_decls: Vec<FnDecl>, st_decls: Vec<StructDecl>) -> Self;
}

pub(super) trait EvePassImmutable: EvePass {
    fn run_pass(&self) -> PassResult;
}

pub(super) trait EvePassMutable: EvePass {
    fn run_pass(&mut self) -> PassResult;
}

pub fn run_passes(fn_: Vec<FnDecl>, st_: Vec<StructDecl>) -> PassResult {
    let p = MainFnExistence::new(fn_, st_);
    let (fn_, st_) = p.run_pass()?;

    let p = AllFnExistence::new(fn_, st_);
    let (fn_, st_) = p.run_pass()?;

    let p = StructInitUniqueField::new(fn_, st_);
    let (fn_, st_) = p.run_pass()?;

    let p = StructFieldMissingAndUnknown::new(fn_, st_);
    let (fn_, st_) = p.run_pass()?;

    let mut p = DeadCodeElimination::new(fn_, st_);
    let (fn_, st_) = p.run_pass()?;

    let p = TypeCheck::new(fn_, st_);
    let (fn_, st_) = p.run_pass()?;

    Ok((fn_, st_))
}
