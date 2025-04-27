mod ast;
mod backend;
mod cc_runtime;
mod cli;
mod emitter;
mod lexer;
mod parser;
mod passes;
mod utils;

use anyhow::{Context, bail};
use backend::Backend;
use backend::qbe_backend::QbeBackend;
use colored::Colorize;
use emitter::Emitter;
use emitter::qbe::QBEEmitter;
use evelin::utils::{MessageType, report_message};
use log::{debug, info};
use parser::Parser;
use std::fs;
use std::time::Instant;

pub fn init() -> anyhow::Result<()> {
    let initial_time = Instant::now();

    let opts = cli::init()?;

    let mut out_files = vec![];
    for f in opts.file {
        let in_src = fs::read_to_string(&f).context("Failed to read input file")?;

        let mut lexer = lexer::Lexer::from(&in_src);
        lexer.start()?;
        debug!("{:?}", &lexer.tokens());

        let mut parser = Parser::from(lexer.tokens());
        parser.parse();
        debug!("{:?}", &parser.struct_decls);
        debug!("{:?}", &parser.fn_decls);
        if parser.errors_count != 0 {
            bail!(
                "Failed to compile due to {} parsing error(s)",
                parser.errors_count
            );
        }

        let (fn_, st) = match passes::run_passes(parser.fn_decls, parser.struct_decls) {
            Ok((fn_, st)) => (fn_, st),
            Err(errs) => {
                for e in &errs {
                    report_message(e.to_string(), MessageType::Error);
                }
                bail!("Failed to compile due to {} error(s)", &errs.len());
            }
        };

        let mut qbe_generator = QBEEmitter::from((&fn_, &st));
        let ir = qbe_generator.emit_ir()?;
        debug!("IR: \n{}", ir);

        let backend = QbeBackend {};
        let obj_code = backend.generate(ir)?;
        debug!("OBJ_CODE: \n{}", obj_code);

        let mut abs_outfile =
            std::path::absolute(f).context("Failed to get absolute path of the input file")?;
        abs_outfile.set_extension("s");
        fs::write(&abs_outfile, obj_code).context("Failed to write qbe output to a file")?;
        out_files.push(abs_outfile);
    }

    // build executable using platform's c compiler
    let out = cc_runtime::Build::default()
        .set_c_compiler(opts.cc)
        .files(&out_files)
        .set_outfile(&opts.out)
        .set_lib_paths(opts.lib_path.unwrap_or(vec![]))
        .set_lib_names(opts.lib_name.unwrap_or(vec![]))
        .set_opt(3)
        .compile()?;

    // delete temporary files.
    for file in out_files {
        fs::remove_file(file)?;
    }

    let elapsed_time = initial_time.elapsed();
    if !out.stderr.is_empty() {
        bail!(String::from_utf8(out.stderr).unwrap_or("c compiler error".to_owned()));
    }

    println!(
        "{} '{}' in {:.2?}",
        "Compiled".green(),
        opts.out,
        elapsed_time
    );

    Ok(())
}

fn main() {
    match init() {
        Ok(()) => info!("Execution finished successfully."),
        Err(err) => {
            eprintln!("{} {:#}", "Error:".red(), err);
        }
    }
}
