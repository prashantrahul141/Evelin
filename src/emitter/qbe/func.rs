use std::collections::HashMap;

use crate::ast::{FnDecl, Stmt};
use crate::emitter::EmitterResult;
use log::trace;
use qbe;

use super::QBEEmitter;

impl QBEEmitter<'_> {
    /// Emits a single function
    pub(super) fn emit_function(&mut self, func: &FnDecl) -> EmitterResult<()> {
        trace!("Emitting a new function: '{}'", &func.name);
        self.scopes.push(HashMap::new());
        let mut func_block = qbe::Function::new(
            qbe::Linkage::public(),
            &func.name,
            func.parameter
                .iter()
                .map(|x| {
                    let ty = qbe::Type::try_from(x.1.clone())?;
                    let val = self.new_var(ty.clone(), x.0.clone())?;
                    Ok((ty, val))
                })
                .collect::<anyhow::Result<Vec<_>>>()?,
            qbe::Type::try_from(func.return_type.clone()).ok(),
        );
        func_block.add_block("start");
        self.emit_function_body(&mut func_block, &func.body)?;

        // add a ret instruction if there isnt one at the end of a function declaration.
        if !func_block.blocks.last_mut().is_some_and(|b| b.jumps()) {
            func_block.add_instr(qbe::Instr::Ret(None));
        }

        trace!("adding new function = {}", &func_block);

        self.module.add_function(func_block);
        self.scopes.pop();
        Ok(())
    }

    /// Emits a single function
    fn emit_function_body(
        &mut self,
        func: &mut qbe::Function<'static>,
        fn_body: &Vec<Stmt>,
    ) -> EmitterResult<()> {
        trace!("emitting function body");
        for stmt in fn_body {
            self.emit_stmt(func, stmt)?;
        }

        Ok(())
    }
}
