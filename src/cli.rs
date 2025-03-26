use std::path::PathBuf;

use clap::Parser;

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum DebugTypes {
    Error,
    Debug,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct ZSCliOptions {
    /// File path
    pub file: PathBuf,

    /// Turn debugging information on
    #[clap(value_enum, default_value_t = DebugTypes::Error)]
    #[arg(short, long)]
    pub debug: DebugTypes,
}
