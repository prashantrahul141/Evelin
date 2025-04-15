use std::{cmp, collections::HashMap};

use crate::ast::StructDecl;
use log::debug;

use super::QBEEmitter;

/// mapping of field-name -> (field-type, field-offset)
pub(super) type StructMeta = HashMap<String, (qbe::Type<'static>, u64)>;

impl QBEEmitter<'_> {
    /// Emits a single function
    pub(super) fn emit_data_def(&mut self, struct_decl: &StructDecl) {
        self.tmp_counter += 1;
        let mut typedef = qbe::TypeDef {
            name: struct_decl.name.clone(),
            align: None,
            items: vec![],
        };

        let mut meta = StructMeta::new();
        let mut offset = 0;
        let mut max_align = 0;

        for field in &struct_decl.fields {
            let ty = qbe::Type::try_from(field.1.clone()).unwrap();
            let field_align = Self::type_alignment(&ty);
            max_align = cmp::max(max_align, field_align);

            // align with the current offset
            offset = Self::align_offset(offset, field_align);

            meta.insert(field.0.clone(), (ty.clone(), offset));
            typedef.items.push((ty.clone(), 1));

            offset += ty.size();
        }

        offset = Self::align_offset(offset, max_align);

        // set typedef's alignment
        typedef.align = Some(max_align);

        self.struct_meta
            .insert(struct_decl.name.clone(), (meta, offset));

        self.type_defs.push(typedef.clone());
        self.module.add_type(typedef);
    }

    // Emits initialization data definition
    pub(super) fn init_data_def(&mut self) {
        debug!("emiting initial data definition");
        self.module.add_data(qbe::DataDef::new(
            qbe::Linkage::private(),
            "___FMT_WORD",
            None,
            vec![
                (qbe::Type::Byte, qbe::DataItem::Str("%d".into())),
                (qbe::Type::Byte, qbe::DataItem::Const(0)),
            ],
        ));

        self.module.add_data(qbe::DataDef::new(
            qbe::Linkage::private(),
            "___FMT_LONG",
            None,
            vec![
                (qbe::Type::Byte, qbe::DataItem::Str("%s".into())),
                (qbe::Type::Byte, qbe::DataItem::Const(0)),
            ],
        ));

        self.module.add_data(qbe::DataDef::new(
            qbe::Linkage::private(),
            "___FMT_SINGLE",
            None,
            vec![
                (qbe::Type::Byte, qbe::DataItem::Str("%f".into())),
                (qbe::Type::Byte, qbe::DataItem::Const(0)),
            ],
        ));

        self.module.add_data(qbe::DataDef::new(
            qbe::Linkage::private(),
            "___FMT_DOUBLE",
            None,
            vec![
                (qbe::Type::Byte, qbe::DataItem::Str("%lf".into())),
                (qbe::Type::Byte, qbe::DataItem::Const(0)),
            ],
        ));
    }
}
