mod backend;
mod cli;
mod utils;

use std::path::{self, Path};

use clap::Parser;
use cli::ZSCliOptions;
use env_logger::Env;
use log::error;

fn init() -> ZSCliOptions {
    let cli = ZSCliOptions::parse();
    let level = match cli.debug {
        cli::DebugTypes::Error => "error",
        cli::DebugTypes::Debug => "debug",
    };

    let env = Env::default().filter_or("ZS_LOG_LEVEL", level);
    env_logger::init_from_env(env);

    let file_path = Path::new(&cli.file);
    if !file_path.is_file() && !file_path.exists() {
        die!("File not found {}", cli.file.to_str().unwrap());
    }

    cli
}

fn main() {
    let options = init();
    let in_file = path::absolute(&options.file).unwrap();
}
