use log::debug;

use super::QBEEmitter;

impl QBEEmitter<'_> {
    /// Emits a single function
    fn emit_data_body(&mut self) {}

    // Emits initialization data definition
    pub(super) fn init_data_def(&mut self) {
        debug!("emiting initial data definition");
        self.module.add_data(qbe::DataDef::new(
            qbe::Linkage::private(),
            "___FMT_INT",
            None,
            vec![
                (qbe::Type::Byte, qbe::DataItem::Str("%ld".into())),
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
