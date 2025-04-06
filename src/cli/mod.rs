use std::io::Write;
use std::path::Path;
use std::{path::PathBuf, str::FromStr};

use clap::Parser;
use env_logger::{Builder, Env};
use log::{error, trace};

use crate::die;

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum DebugTypes {
    Error,
    Debug,
    Trace,
}

#[derive(Clone, Debug)]
pub enum InFile {
    File(PathBuf),
    Stdin,
}

impl FromStr for InFile {
    type Err = anyhow::Error;

    fn from_str(v: &str) -> Result<InFile, anyhow::Error> {
        if v == "-" {
            Ok(InFile::Stdin)
        } else {
            Ok(InFile::File(PathBuf::from_str(v).unwrap()))
        }
    }
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct EveCliOptions {
    /// File path, use '-' to start repl mode.
    pub file: InFile,

    /// Turn debugging information on
    #[clap(value_enum, default_value_t = DebugTypes::Error)]
    #[arg(short, long)]
    pub debug: DebugTypes,
}

pub fn init() -> EveCliOptions {
    let cli = EveCliOptions::parse();
    let level = match cli.debug {
        DebugTypes::Error => "error",
        DebugTypes::Debug => "debug",
        DebugTypes::Trace => "trace",
    };

    let env = Env::default().filter_or("EVE_LOG_LEVEL", level);

    Builder::from_env(env)
        .format(|buf, record| {
            let warn_style = buf.default_level_style(record.level());
            writeln!(
                buf,
                "{warn_style}{}:{}:{}L:{warn_style:#} {}",
                record.level(),
                record.file().unwrap(),
                record.line().unwrap(),
                record.args()
            )
        })
        .init();

    match &cli.file {
        InFile::File(f) => {
            let file_path = Path::new(f);
            if !file_path.is_file() && !file_path.exists() {
                die!("File not found {}", f.to_str().unwrap());
            }
        }
        InFile::Stdin => {
            trace!("repl mode.");
        }
    };

    cli
}
