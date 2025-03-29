mod backend;
mod cli;
mod lexer;
mod token;
mod utils;

use std::io::{self, Write};
use std::{fs, path::Path};

use clap::Parser;
use cli::ZSCliOptions;
use env_logger::Env;
use lexer::Lexer;
use log::{error, trace};

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
        cli::InFile::Stdin => loop {
            let mut in_line = String::new();
            print!(">>> ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut in_line).unwrap();

            let mut lexer = Lexer::new(&in_line);
            lexer.start();
            dbg!(lexer.tokens);
        },
    };
}
