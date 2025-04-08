use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use anyhow::bail;
use clap::Parser;
use env_logger::{Builder, Env};

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum DebugTypes {
    Error,
    Debug,
    Trace,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct EveCliOptions {
    /// Evelin source file path
    pub file: PathBuf,

    /// Turn debugging information on
    #[clap(value_enum, default_value_t = DebugTypes::Error)]
    #[arg(short, long)]
    pub debug: DebugTypes,
}

pub fn init() -> anyhow::Result<EveCliOptions> {
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

    let file_path = Path::new(&cli.file);
    if !file_path.is_file() || !file_path.exists() {
        bail!("File '{}' not found", cli.file.to_str().unwrap());
    }

    Ok(cli)
}
