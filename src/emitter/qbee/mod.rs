mod data;
mod expr;
mod func;
mod stmt;
mod utils;

use std::collections::HashMap;

use crate::ast::{FnDecl, StructDecl};
use crate::emitter::EmitterResult;
use data::StructMeta;
use log::{debug, info};
use qbe;

use super::Emitter;

/// Public struct for the QBE IR Emitter.
pub struct QBEEmitter<'a> {
    /// Counts total number of temporaries created.
    tmp_counter: usize,

    /// Function declarations
    fn_decls: &'a Vec<FnDecl>,

    /// Struct declarations
    struct_decls: &'a Vec<StructDecl>,

    /// Scopes for variables.
    scopes: Vec<HashMap<String, (qbe::Type<'static>, qbe::Value)>>,

    /// Scopes for loops.
    loop_scopes: Vec<usize>,

    /// Type defs emitted for this module
    type_defs: Vec<qbe::TypeDef<'static>>,

    /// struct meta data struct_name -> (struct-meta, struct-size)
    struct_meta: HashMap<String, (StructMeta, u64)>,

    /// Current module.
    /// This is usually 1 module per file basis.
    module: qbe::Module<'a>,
}

/// Impl From for QBEEmitter.
impl<'a> From<(&'a Vec<FnDecl>, &'a Vec<StructDecl>)> for QBEEmitter<'a> {
    fn from(decls: (&'a Vec<FnDecl>, &'a Vec<StructDecl>)) -> Self {
        info!("creating new QBEEmitter instance");
        Self {
            tmp_counter: 0,
            fn_decls: decls.0,
            struct_decls: decls.1,
            scopes: vec![],
            loop_scopes: vec![],
            type_defs: vec![],
            struct_meta: HashMap::new(),
            module: qbe::Module::new(),
        }
    }
}

/// Impl Emitter trait for QBEEmitter.
impl Emitter for QBEEmitter<'_> {
    fn emit_ir(&mut self) -> EmitterResult<String> {
        debug!("start emitting qbe ir");
        self.emit_data_defs();
        self.emit_functions()?;
        Ok(self.module.to_string())
    }
}

/// More impl for QBEEmitter.
impl QBEEmitter<'_> {
    /// Emits all parsed structs
    fn emit_data_defs(&mut self) {
        self.init_data_def();
        for struc in self.struct_decls {
            self.emit_data_def(struc);
        }
    }
    /// Emits all parsed functions
    fn emit_functions(&mut self) -> EmitterResult<()> {
        debug!("Emitting functions");
        for func in self.fn_decls {
            self.emit_function(func)?;
        }
        Ok(())
    }
}
