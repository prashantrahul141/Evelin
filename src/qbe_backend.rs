use core::panic;
use std::{
    fs::{self, File},
    io::Write,
    os::unix::fs::PermissionsExt,
    path::PathBuf,
    process::Command,
};

use log::error;

use crate::die;

const QBE_BINARY_DATA: &[u8] = include_bytes!("./external/qbe-1.2/qbe");
const QBE_EXECUTABLE_PATH: &str = "/tmp/qbe";

pub struct QbeBackend {
    pub out_file: PathBuf,
    in_file: PathBuf,
}

impl<T: Into<PathBuf> + Clone + Copy> From<T> for QbeBackend {
    fn from(in_file: T) -> Self {
        let mut exe_file = match File::create(QBE_EXECUTABLE_PATH) {
            Ok(f) => f,
            Err(e) => {
                die!("Failed to create /tmp/qbe file. {}", e);
            }
        };

        exe_file.write_all(QBE_BINARY_DATA).unwrap_or_else(|e| {
            die!("Failed to write to /tmp/qbe file: {}", e);
        });

        let metadata = match exe_file.metadata() {
            Ok(m) => m,
            Err(e) => {
                die!(
                    "Failed to get metadata for qbe binary file at /tmp/qbe : {}",
                    e
                );
            }
        };

        let mut permissions = metadata.permissions();
        permissions.set_mode(0o677);

        let mut out_file = in_file.into();
        out_file.set_extension("s");

        Self {
            in_file: in_file.into(),
            out_file,
        }
    }
}

impl QbeBackend {
    pub fn run(&self) -> String {
        match Command::new(QBE_EXECUTABLE_PATH)
            .args(&[
                "-o",
                self.out_file.to_str().unwrap(),
                self.in_file.to_str().unwrap(),
            ])
            .output()
        {
            Ok(_) => {}
            Err(e) => {
                die!("Failed to execute qbe binary: {}", e);
            }
        };

        fs::read_to_string(&self.out_file).expect("Failed to read output file.")
    }
}
