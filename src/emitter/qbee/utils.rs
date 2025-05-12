use anyhow::{anyhow, bail};
use log::{error, trace};

use crate::{
    ast::{DType, EveTypes, Token, TokenType},
    die,
    emitter::EmitterResult,
};

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

    /// Creates a new global, returns the generated qbe::Value
    pub(super) fn new_glob_name(&mut self) -> String {
        self.tmp_counter += 1;
        trace!("creating new glob = @glob.{}", self.tmp_counter);
        format!("glob.{}", self.tmp_counter)
    }

    /// Get maximum alignment of a type
    pub(super) fn type_alignment(ty: &qbe::Type) -> u64 {
        match ty {
            qbe::Type::Byte | qbe::Type::SignedByte | qbe::Type::UnsignedByte => 1,
            qbe::Type::Halfword | qbe::Type::SignedHalfword | qbe::Type::UnsignedHalfword => 2,
            qbe::Type::Word | qbe::Type::Single => 4,
            qbe::Type::Long | qbe::Type::Double => 8,
            qbe::Type::Aggregate(td) => td
                .items
                .iter()
                .map(|(item_ty, _)| QBEEmitter::type_alignment(item_ty))
                .max()
                .unwrap_or(1),
            qbe::Type::Zero => 1,
        }
    }

    /// Get next round offset value depending on alignment
    pub(super) fn align_offset(offset: u64, alignment: u64) -> u64 {
        (offset + alignment - 1) & !(alignment - 1)
    }
}

impl TryFrom<DType> for qbe::Type<'_> {
    type Error = anyhow::Error;

    fn try_from(value: DType) -> Result<Self, Self::Error> {
        match value {
            DType::Primitive(EveTypes::Int) => Ok(qbe::Type::Word),
            DType::Primitive(EveTypes::Float) => Ok(qbe::Type::Double),
            DType::Primitive(EveTypes::String) => Ok(qbe::Type::Long),
            DType::Primitive(EveTypes::Void) => Ok(qbe::Type::Word),
            DType::Derived(_) => Err(anyhow!("qbe::Type::TryFrom<EveTypes> recieved type = Void")),
        }
    }
}

impl TryFrom<&DType> for qbe::Type<'_> {
    type Error = anyhow::Error;

    fn try_from(value: &DType) -> Result<Self, Self::Error> {
        match value {
            DType::Primitive(EveTypes::Int) => Ok(qbe::Type::Word),
            DType::Primitive(EveTypes::Float) => Ok(qbe::Type::Double),
            DType::Primitive(EveTypes::String) => Ok(qbe::Type::Long),
            DType::Primitive(EveTypes::Void) => Ok(qbe::Type::Word),
            DType::Derived(_) => Err(anyhow!("qbe::Type::TryFrom<EveTypes> recieved type = Void")),
        }
    }
}

impl TryFrom<EveTypes> for qbe::Type<'_> {
    type Error = anyhow::Error;

    fn try_from(value: EveTypes) -> Result<Self, Self::Error> {
        match value {
            EveTypes::Int => Ok(qbe::Type::Word),
            EveTypes::Float => Ok(qbe::Type::Double),
            EveTypes::String => Ok(qbe::Type::Long),
            EveTypes::Void => Err(anyhow!("qbe::Type::TryFrom<EveTypes> recieved type = Void")),
        }
    }
}

impl TryFrom<Token> for qbe::Type<'_> {
    type Error = anyhow::Error;

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value.ttype {
            TokenType::TypeInt => Ok(qbe::Type::Word),
            TokenType::TypeFloat => Ok(qbe::Type::Double),
            TokenType::TypeVoid => Err(anyhow!("qbe::Type::TryFrom recieved type = TypeVoid")),
            v => {
                die!("qbe::Value::from failed, recieved token type: {}", v);
            }
        }
    }
}
