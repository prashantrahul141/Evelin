use anyhow::{anyhow, bail};
use log::{error, trace};

use crate::{ast::TokenType, die, emitter::EmitterResult};

use super::QBEEmitter;

impl QBEEmitter<'_> {
    /// Creates a new bindings bound to current scope.
    pub(super) fn new_var(
        &mut self,
        ty: qbe::Type<'static>,
        name: String,
    ) -> EmitterResult<qbe::Value> {
        if self.get_var(&name).is_ok() {
            bail!("Re-declaration of variable : {}", name);
        }

        let tmp = self.new_tmp();
        let scope = self
            .scopes
            .last_mut()
            .expect("Expected last scope to be present");
        scope.insert(name, (ty.to_owned(), tmp.to_owned()));

        Ok(tmp)
    }

    /// Retrieves an existing bind
    /// Searches in reverse order of scopes.
    pub(super) fn get_var(
        &mut self,
        name: &String,
    ) -> EmitterResult<&(qbe::Type<'static>, qbe::Value)> {
        self.scopes
            .iter()
            .rev()
            .filter_map(|x| x.get(name))
            .next()
            .ok_or_else(|| anyhow!("undefined variable: {}", name))
    }

    /// Creates a new temporary, returns the generated qbe::Value
    pub(super) fn new_tmp(&mut self) -> qbe::Value {
        self.tmp_counter += 1;
        trace!("creating new tmp = %tmp.{}", self.tmp_counter);
        qbe::Value::Temporary(format!("tmp.{}", self.tmp_counter))
    }
}

impl TryFrom<TokenType> for qbe::Type<'_> {
    type Error = anyhow::Error;

    fn try_from(value: TokenType) -> Result<Self, Self::Error> {
        match value {
            TokenType::TypeI64 => Ok(qbe::Type::Long),
            TokenType::TypeF64 => Ok(qbe::Type::Double),
            TokenType::TypeVoid => Err(anyhow!("qbe::Type::TryFrom recieved type = TypeVoid")),
            v => {
                die!("qbe::Value::from failed, recieved token type: {}", v);
            }
        }
    }
}
