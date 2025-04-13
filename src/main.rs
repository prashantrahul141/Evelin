mod ast;
mod backend;
mod cc;
mod cli;
mod emitter;
mod lexer;
mod parser;
mod utils;

use anyhow::{Context, bail};
use backend::Backend;
use backend::qbe_backend::QbeBackend;
use colored::Colorize;
use emitter::Emitter;
use emitter::qbe::QBEEmitter;
use log::{debug, error, info};
use parser::Parser;
use std::{fs, path::PathBuf};

pub fn init() -> anyhow::Result<()> {
    let opts = cli::init()?;

    let mut out_files = vec![];
    for f in opts.file {
        let in_src = fs::read_to_string(&f).context("Failed to read input file")?;

        let mut lexer = lexer::Lexer::from(&in_src);
        lexer.start();
        debug!("{:?}", &lexer.tokens());

        let mut parser = Parser::from(lexer.tokens());
        parser.parse();
        debug!("{:?}", &parser.struct_decls);
        debug!("{:?}", &parser.fn_decls);

        let mut qbe_generator = QBEEmitter::from((&parser.fn_decls, &parser.struct_decls));
        let ir = qbe_generator.emit_ir()?;
        debug!("IR: {}", ir);

        let backend = QbeBackend::default();
        let obj_code = backend.generate(ir)?;
        debug!("OBJ_CODE: {}", obj_code);

        let mut abs_outfile = PathBuf::from(
            std::path::absolute(f).context("Failed to get absolute path of the input file")?,
        );
        abs_outfile.set_extension("s");
        fs::write(&abs_outfile, obj_code).context("Failed to write qbe output to a file")?;
        out_files.push(abs_outfile);
    }

    // build executable using platform's c compiler
    let out = cc::Build::new()
        .set_c_compiler(opts.cc)
        .files(&out_files)
        .set_outfile(opts.out)
        .set_lib_paths(opts.lib_path.unwrap_or(vec![]))
        .set_lib_names(opts.lib_name.unwrap_or(vec![]))
        .set_opt(3)
        .compile()?;

    if out.stderr.len() != 0 {
        bail!(String::from_utf8(out.stderr).unwrap_or("c compiler error".to_owned()));
    }

    // delete temprory files.
    for file in out_files {
        fs::remove_file(file)?;
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
