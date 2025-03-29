mod backend;
mod cli;
mod lexer;
mod token;
mod utils;

use std::{fs, path::Path};

use clap::Parser;
use cli::ZSCliOptions;
use env_logger::Env;
use lexer::Lexer;
use log::{error, trace};
use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;

fn init() -> ZSCliOptions {
    let cli = ZSCliOptions::parse();
    let level = match cli.debug {
        cli::DebugTypes::Error => "error",
        cli::DebugTypes::Debug => "debug",
    };

    let env = Env::default().filter_or("ZS_LOG_LEVEL", level);
    env_logger::init_from_env(env);

    match &cli.file {
        cli::InFile::File(f) => {
            let file_path = Path::new(f);
            if !file_path.is_file() && !file_path.exists() {
                die!("File not found {}", f.to_str().unwrap());
            }
        }
        cli::InFile::Stdin => {
            trace!("repl mode.");
        }
    };

    cli
}

fn main() {
    let options = init();
    match options.file {
        // file mode.
        cli::InFile::File(f) => {
            let in_src = fs::read_to_string(f).unwrap();
            let mut lexer = Lexer::new(&in_src);
            lexer.start();
            dbg!(lexer.tokens);
        }
        // repl mode.
        cli::InFile::Stdin => {
            let mut rl = DefaultEditor::new().unwrap();
            loop {
                let readline = rl.readline(">>> ");
                match readline {
                    Ok(line) => {
                        let _ = rl.add_history_entry(line.as_str());
                        let mut lexer = Lexer::new(&line);
                        lexer.start();
                        dbg!(lexer.tokens);
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
