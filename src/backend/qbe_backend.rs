use core::panic;
use std::{
    fs::File,
    io::Write,
    os::unix::fs::PermissionsExt,
    process::{Command, Stdio},
};

use anyhow::Context;
use log::error;

use crate::die;

use super::Backend;

const QBE_BINARY_DATA: &[u8] = include_bytes!("../external/qbe-1.2/qbe");

#[cfg(target_os = "macos")]
const QBE_EXECUTABLE_PATH: &str = "./qbe";

#[cfg(target_os = "linux")]
const QBE_EXECUTABLE_PATH: &str = "/tmp/qbe";

/// Public qbe backend struct.
pub struct QbeBackend {}

impl Default for QbeBackend {
    fn default() -> Self {
        let mut exe_file = File::create(QBE_EXECUTABLE_PATH).unwrap_or_else(|e| {
            die!("Failed to create /tmp/qbe file. {}", e);
        });

        exe_file.write_all(QBE_BINARY_DATA).unwrap_or_else(|e| {
            die!("Failed to write to /tmp/qbe file: {}", e);
        });

        let metadata = exe_file.metadata().unwrap_or_else(|e| {
            die!(
                "Failed to get metadata for qbe binary file at /tmp/qbe : {}",
                e
            );
        });

        metadata.permissions().set_mode(0o677);
        Self {}
    }
}

impl Backend for QbeBackend {
    fn generate(&self, ir: String) -> Result<String, anyhow::Error> {
        let mut child = Command::new(QBE_EXECUTABLE_PATH)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .context("Failed to spawn child qbe process.")?;

        let mut stdin = child
            .stdin
            .take()
            .context("Failed to open stdin for qbe child process.")?;

        std::thread::spawn(move || -> anyhow::Result<()> {
            stdin
                .write_all(ir.as_bytes())
                .context("Failed to write to stdin for qbe child process.")
        });

        let output = child.wait_with_output()?;
        Ok(String::from_utf8(output.stdout)?)
    }
}
