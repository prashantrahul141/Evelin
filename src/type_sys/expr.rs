use anyhow::{anyhow, bail};

use crate::ast::{
    BinExpr, CallExpr, DType, EveTypes, Expr, FieldAccessExpr, LiteralExpr, LiteralValue,
    NativeCallExpr, UnaryExpr, VariableExpr,
};

use super::TypeSystem;

impl TypeSystem<'_> {
    pub(super) fn check_expr(&self, expr: &mut Expr) -> anyhow::Result<DType> {
        match expr {
            Expr::Binary(bin) => self.check_binary(bin),
            Expr::Call(call) => self.check_call(call),
            Expr::FieldAccess(fiac) => self.check_field_access(fiac),
            Expr::NativeCall(nacall) => self.check_native_call(nacall),
            Expr::Unary(un) => self.check_unary(un),
            Expr::Grouping(group) => self.check_expr(&mut group.value),
            Expr::Variable(var) => self.check_var(var),
            Expr::Literal(lit) => Ok(self.check_literal(lit)),
        }
    }

    fn check_binary(&self, bin: &mut BinExpr) -> anyhow::Result<DType> {
        let left = self.check_expr(&mut bin.left)?;
        let right = self.check_expr(&mut bin.right)?;

        let ty = match (left, right) {
            (DType::Primitive(EveTypes::Int), DType::Primitive(EveTypes::Int)) => {
                DType::Primitive(EveTypes::Int)
            }

            (DType::Primitive(EveTypes::Int), DType::Primitive(EveTypes::Float)) => {
                bin.left.node_type = Some(DType::Primitive(EveTypes::Float));
                DType::Primitive(EveTypes::Float)
            }
            (DType::Primitive(EveTypes::Int), DType::Primitive(EveTypes::String)) => bail!(
                "{} operation cannot be applied between Int and String, line {}",
                &bin.op,
                bin.metadata.line
            ),
            (DType::Primitive(EveTypes::Int), DType::Primitive(EveTypes::Void)) => bail!(
                "{} operation cannot be applied between Int and Void, line {}",
                &bin.op,
                bin.metadata.line
            ),
            (DType::Primitive(EveTypes::Float), DType::Primitive(EveTypes::Int)) => {
                bin.right.node_type = Some(DType::Primitive(EveTypes::Float));
                DType::Primitive(EveTypes::Float)
            }
            (DType::Primitive(EveTypes::Float), DType::Primitive(EveTypes::Float)) => {
                DType::Primitive(EveTypes::Float)
            }
            (DType::Primitive(EveTypes::Float), DType::Primitive(EveTypes::String)) => bail!(
                "{} operation cannot be applied between Float and String, line {}",
                &bin.op,
                bin.metadata.line
            ),
            (DType::Primitive(EveTypes::Float), DType::Primitive(EveTypes::Void)) => bail!(
                "{} operation cannot be applied between Float and Void, line {}",
                &bin.op,
                bin.metadata.line
            ),
            (DType::Primitive(EveTypes::String), DType::Primitive(EveTypes::Int)) => bail!(
                "{} operation cannot be applied between String and Int, line {}",
                &bin.op,
                bin.metadata.line
            ),
            (DType::Primitive(EveTypes::String), DType::Primitive(EveTypes::Float)) => bail!(
                "{} operation cannot be applied between String and Float, line {}",
                &bin.op,
                bin.metadata.line
            ),
            (DType::Primitive(EveTypes::String), DType::Primitive(EveTypes::String)) => bail!(
                "{} operation cannot be applied between String and String, line {}",
                &bin.op,
                bin.metadata.line
            ),
            (DType::Primitive(EveTypes::String), DType::Primitive(EveTypes::Void)) => bail!(
                "{} operation cannot be applied between String and Void, line {}",
                &bin.op,
                bin.metadata.line
            ),

            (DType::Primitive(EveTypes::Void), _) => unreachable!(),
            (DType::Derived(derived_name), DType::Primitive(primitive_ty)) => bail!(
                "{} operation cannot be applied between {} and {}, line {}",
                &bin.op,
                derived_name,
                primitive_ty,
                bin.metadata.line
            ),
            (DType::Derived(_), DType::Derived(_)) => {
                bail!("Struct method overloading not available",)
            }
            (DType::Primitive(EveTypes::Int), DType::Derived(_))
            | (DType::Primitive(EveTypes::Float), DType::Derived(_))
            | (DType::Primitive(EveTypes::String), DType::Derived(_)) => {
                bail!(
                    "Operations between derived and primitive type not available, line {}",
                    bin.metadata.line
                )
            }
        };
        bin.metadata.node_type = Some(ty.clone());
        Ok(ty)
    }

    fn check_call(&self, call: &mut CallExpr) -> anyhow::Result<DType> {
        let fn_name = match &call.callee {
            Expr::Variable(var) => &var.name,
            _ => bail!("callee is not a identifier, line {}", call.metadata.line),
        };
        let fn_decl = match self.fn_decls.iter().find(|x| &x.name == fn_name) {
            Some(fn_decl) => fn_decl,
            None => bail!(
                "Function '{}' not defined, line {}",
                fn_name,
                call.metadata.line
            ),
        };

        call.metadata.node_type = Some(fn_decl.return_type.clone());
        Ok(fn_decl.return_type.clone())
    }

    fn check_field_access(&self, field_access: &mut FieldAccessExpr) -> anyhow::Result<DType> {
        let var_name = match &field_access.parent {
            Expr::Variable(var) => &var.name,
            _ => unreachable!(
                "field access's parent is not a variable, line {}",
                field_access.metadata.line
            ),
        };

        let var_type = self.get_env(var_name).ok_or(anyhow!(
            "Variable '{}' not defined, line {}",
            &var_name,
            field_access.metadata.line
        ))?;

        let st_name = match var_type {
            DType::Primitive(_) => bail!(
                "'{}' is not a struct instance, line {}",
                &var_name,
                field_access.metadata.line
            ),
            DType::Derived(st_name) => st_name,
        };

        let st_decl = self
            .st_decls
            .iter()
            .find(|x| &x.name == st_name)
            .ok_or(anyhow!(
                "Struct '{}' not defined, line {}",
                var_name,
                field_access.metadata.line
            ))?;

        let field = st_decl
            .fields
            .iter()
            .find(|x| x.field_name == field_access.field)
            .ok_or(anyhow!(
                "Struct '{}' as no field '{}', line {}",
                &var_name,
                &field_access.field,
                field_access.metadata.line
            ))?;

        let ty = DType::Primitive(EveTypes::try_from(&field.field_type)?);
        field_access.metadata.node_type = Some(ty.clone());
        Ok(ty)
    }

    fn check_native_call(&self, na_call: &mut NativeCallExpr) -> anyhow::Result<DType> {
        let ty = DType::Primitive(EveTypes::Int);
        na_call.metadata.node_type = Some(ty.clone());
        Ok(ty)
    }

    fn check_unary(&self, un: &mut UnaryExpr) -> anyhow::Result<DType> {
        let ty = self.check_expr(&mut un.operand)?;
        un.metadata.node_type = Some(ty.clone());
        Ok(ty)
    }

    fn check_var(&self, var: &mut VariableExpr) -> anyhow::Result<DType> {
        if let Some(ty) = self.get_env(&var.name) {
            var.metadata.node_type = Some(ty.clone());
            return Ok(ty.to_owned());
        }

        bail!(
            "Variable '{}' is not defined, line {}",
            &var.name,
            var.metadata.line
        )
    }

    fn check_literal(&self, literal: &mut LiteralExpr) -> DType {
        let ty = DType::Primitive(match literal.value {
            LiteralValue::NumberFloat(_) => EveTypes::Float,
            LiteralValue::NumberInt(_) => EveTypes::Int,
            LiteralValue::String(_) => EveTypes::String,
            LiteralValue::Boolean(_) => EveTypes::Int,
            LiteralValue::Null => EveTypes::Int,
        });

        literal.metadata.node_type = Some(ty.clone());
        ty
    }
}
