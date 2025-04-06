#![allow(dead_code, unused_imports)]

pub mod ast;
mod backend;
mod cli;
mod emitter;
pub mod lexer;
pub mod parser;
mod utils;

use backend::Backend;
use backend::qbe_backend::QbeBackend;
use emitter::Emitter;
use emitter::qbe_emitter::QBEEmitter;
use log::error;
use parser::Parser;
use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;
use std::fs;

pub fn init() {
    let options = cli::init();
    let backend = QbeBackend::default();
    match options.file {
        // file mode.
        cli::InFile::File(f) => {
            let in_src = fs::read_to_string(f).unwrap();
            let mut lexer = lexer::Lexer::from(&in_src);
            lexer.start();
            dbg!(&lexer.tokens());
            let mut parser = Parser::from(lexer.tokens());
            parser.parse();
            dbg!(&parser.fn_decls);
            dbg!(&parser.struct_decls);
            // let mut qbe_generator = QBEEmitter::from(&parser.stmts);
            // let ir = qbe_generator.emit_ir().unwrap();
            // println!("{}", ir);
        }
        // repl mode.
        cli::InFile::Stdin => {
            let mut rl = DefaultEditor::new().unwrap();
            loop {
                let readline = rl.readline(">>> ");
                match readline {
                    Ok(line) => {
                        let _ = rl.add_history_entry(line.as_str());
                        let mut lexer = lexer::Lexer::from(&line);
                        lexer.start();
                        let mut parser = Parser::from(lexer.tokens());
                        parser.parse();
                    }
                    Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                        println!("Interrupted");
                        break;
                    }
                    Err(err) => {
                        die!("Failed to readline : {:?}", err);
                    }
                }
            }
        }
    };
}
