use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use clap::CommandFactory;
use clap::Parser;
use clap::error::ErrorKind;
use colored::Colorize;
use env_logger::{Builder, Env};

const EVE_FILE_EXTENSION: &str = "eve";

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum DebugTypes {
    Error,
    Debug,
    Trace,
}

#[derive(Parser, Debug)]
#[command(arg_required_else_help = true)]
#[command(version, about= "The Evelin Programming Language", long_about = None)]
pub struct EveCliOptions {
    /// Evelin source files path
    pub file: Vec<PathBuf>,

    /// C compiler
    #[clap(default_value = "cc")]
    #[arg(short, long)]
    pub cc: String,

    /// Turn debugging information on
    #[clap(value_enum, default_value_t = DebugTypes::Error)]
    #[arg(short, long)]
    pub debug: DebugTypes,

    /// Out file name
    #[clap(default_value = "out")]
    #[arg(short, long)]
    pub out: String,

    /// External library name passed to the linker as -l<lib1> -l<lib2>
    #[arg(short, long = "lib_name", value_delimiter = ' ', num_args = 1.. )]
    pub lib_name: Option<Vec<String>>,

    /// External library directory passed to the linker as -L<path_1> -L<path_2>
    #[arg(short = 'L', long = "lib_path", value_delimiter = ' ', num_args = 1.. )]
    pub lib_path: Option<Vec<String>>,
}

pub fn init() -> anyhow::Result<EveCliOptions> {
    let cli = EveCliOptions::parse();
    let mut cmd = EveCliOptions::command();

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

    for f in &cli.file {
        let file_path = Path::new(f);

        let ext_matches =
            file_path.extension().and_then(|ext| ext.to_str()) == Some(EVE_FILE_EXTENSION);

        let tmp_str = f.to_str().unwrap();
        if !file_path.is_file() || !file_path.exists() {
            cmd.error(
                ErrorKind::ValueValidation,
                format!("File '{}' not found.", tmp_str),
            )
            .exit();
        }

        if !ext_matches {
            cmd.error(
                ErrorKind::ValueValidation,
                format!(
                    "Incorrect file type for {}. Expected a {} file.",
                    tmp_str.red(),
                    ".eve".green()
                ),
            )
            .exit();
        }
    }

    Ok(cli)
}
