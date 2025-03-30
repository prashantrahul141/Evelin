use std::{path::PathBuf, str::FromStr};

use clap::Parser;

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum DebugTypes {
    Error,
    Debug,
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
pub struct ZSCliOptions {
    /// File path, use '-' to start repl mode.
    pub file: InFile,

    /// Turn debugging information on
    #[clap(value_enum, default_value_t = DebugTypes::Error)]
    #[arg(short, long)]
    pub debug: DebugTypes,
}
