mod ast;
mod backend;
mod cli;
mod emitter;
mod lexer;
mod parser;
mod token;
mod utils;

use backend::Backend;
use backend::qbe_backend::QbeBackend;
use emitter::Emitter;
use emitter::qbe_emitter::QBEEmitter;
use lexer::Lexer;
use log::error;
use parser::parser::Parser;
use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;
use std::fs;

fn main() {
    let options = cli::init();
    let backend = QbeBackend::default();
    match options.file {
        // file mode.
        cli::InFile::File(f) => {
            let in_src = fs::read_to_string(f).unwrap();
            let mut lexer = lexer::Lexer::from(&in_src);
            lexer.start();
            let mut parser = Parser::from(lexer.tokens());
            parser.parse();
            let mut qbe_generator = QBEEmitter::from(&parser.stmts);
            let ir = qbe_generator.emit_ir().unwrap();
            println!("{}", ir);
        }
        // repl mode.
        cli::InFile::Stdin => {
            let mut rl = DefaultEditor::new().unwrap();
            loop {
                let readline = rl.readline(">>> ");
                match readline {
                    Ok(line) => {
                        let _ = rl.add_history_entry(line.as_str());
                        let mut lexer = Lexer::from(&line);
                        lexer.start();
                        let mut parser = Parser::from(lexer.tokens());
                        parser.parse();
                        let mut qbe_generator = QBEEmitter::from(&parser.stmts);
                        let ir = qbe_generator.emit_ir().unwrap();
                        println!("{}", backend.generate(ir).unwrap());
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
