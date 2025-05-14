mod ast;
mod backend;
mod cc_runtime;
mod cli;
mod emitter;
mod lexer;
mod parser;
mod passes;
mod type_sys;
mod utils;

use anyhow::{Context, bail};
use backend::Backend;
use backend::qbe_backend::QbeBackend;
use colored::Colorize;
use emitter::Emitter;
use emitter::qbee::QBEEmitter;
use evelin::utils::{ErrorType, MessageType, report_message};
use log::{debug, info};
use parser::Parser;
use std::fs;
use std::time::Instant;

pub fn init() -> anyhow::Result<()> {
    let initial_time = Instant::now();

    let opts = cli::init()?;

    let mut fn_decls = vec![];
    let mut struct_decls = vec![];

    for f in &opts.file {
        let in_src = fs::read_to_string(f).context("Failed to read input file")?;

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

        fn_decls.append(&mut parser.fn_decls);
        struct_decls.append(&mut parser.struct_decls);
    }

    debug!("collective = \n {:?}", struct_decls);
    debug!("collective = \n {:?}", fn_decls);

    let (mut fn_decls, mut struct_decls) = match passes::run_passes(fn_decls, struct_decls) {
        Ok((fn_, st)) => (fn_, st),
        Err(errs) => {
            for e in &errs {
                report_message(e.to_string(), MessageType::Error(ErrorType::None));
            }
            bail!("Failed to compile due to {} error(s)", &errs.len());
        }
    };

    debug!("After passes = \n {:?}", struct_decls);
    debug!("After passes = \n {:?}", fn_decls);

    let mut type_sys = type_sys::TypeSystem::new(&mut fn_decls, &mut struct_decls);
    type_sys.check();
    if type_sys.errors_count != 0 {
        bail!(
            "Failed to compile due to {} type error(s)",
            type_sys.errors_count
        );
    }

    debug!("After type check = \n {:?}", type_sys.st_decls());
    debug!("After type check = \n {:?}", type_sys.fn_decls());

    let mut qbe_generator = QBEEmitter::from((&fn_decls, &struct_decls));
    let ir = qbe_generator.emit_ir()?;
    debug!("IR: \n{}", ir);

    let backend = QbeBackend {};
    let obj_code = backend.generate(ir)?;
    debug!("OBJ_CODE: \n{}", obj_code);

    let mut abs_outfile = std::path::absolute(opts.file.first().unwrap())?;
    abs_outfile.set_extension("s");
    fs::write(&abs_outfile, obj_code).context("Failed to write qbe output to a file")?;

    // build executable using platform's c compiler
    let out = cc_runtime::Build::default()
        .set_c_compiler(opts.cc)
        .file(&abs_outfile)
        .set_outfile(&opts.out)
        .set_lib_paths(opts.lib_path.unwrap_or(vec![]))
        .set_lib_names(opts.lib_name.unwrap_or(vec![]))
        .set_opt(3)
        .compile()?;

    // delete temporary file.
    fs::remove_file(&abs_outfile)?;

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
use std::process::ExitCode;
fn main() -> ExitCode {
    match init() {
        Ok(()) => {
            info!("Execution finished successfully.");
            ExitCode::SUCCESS
        }
        Err(err) => {
            report_message(format!("{:#}", err), MessageType::Error(ErrorType::None));
            ExitCode::FAILURE
        }
    }
}
