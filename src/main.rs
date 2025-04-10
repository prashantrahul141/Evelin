mod ast;
mod backend;
mod cli;
mod emitter;
mod lexer;
mod parser;
mod utils;

use backend::Backend;
use backend::qbe_backend::QbeBackend;
use colored::Colorize;
use emitter::Emitter;
use emitter::qbe::QBEEmitter;
use log::{debug, error, info};
use parser::Parser;
use std::fs;

pub fn init() -> anyhow::Result<()> {
    let opts = cli::init()?;

    for f in opts.file {
        let in_src = fs::read_to_string(f).unwrap();

        let mut lexer = lexer::Lexer::from(&in_src);
        lexer.start();
        debug!("{:?}", &lexer.tokens());

        let mut parser = Parser::from(lexer.tokens());
        parser.parse();
        debug!("{:?}", &parser.struct_decls);
        debug!("{:?}", &parser.fn_decls);

        let mut qbe_generator = QBEEmitter::from((&parser.fn_decls, &parser.struct_decls));
        let ir = qbe_generator.emit_ir()?;
        debug!("{:?}", ir);

        // let backend = QbeBackend::default();
    }

    Ok(())
}

fn main() {
    match init() {
        Ok(()) => info!("Execution finished successfully."),
        Err(err) => {
            error!("Execution failed : {:?}", err);
            println!("{} {:#}", "Error:".red(), err);
        }
    }
}
