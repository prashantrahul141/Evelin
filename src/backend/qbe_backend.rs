use std::{
    fs::File,
    io::Write,
    os::unix::fs::PermissionsExt,
    process::{Command, Stdio},
};

use anyhow::Context;

use super::Backend;

const QBE_BINARY_DATA: &[u8] = include_bytes!("../../external/qbe-1.2/qbe");

#[cfg(target_os = "macos")]
const QBE_EXECUTABLE_PATH: &str = "./qbe";

#[cfg(target_os = "linux")]
const QBE_EXECUTABLE_PATH: &str = "/tmp/qbe";

/// Public qbe backend struct.
pub struct QbeBackend;

impl QbeBackend {
    pub fn new() -> anyhow::Result<Self> {
        let mut exe_file = File::create(QBE_EXECUTABLE_PATH)
            .context(format!("Failed to create {} file", QBE_EXECUTABLE_PATH))?;

        exe_file
            .write_all(QBE_BINARY_DATA)
            .context(format!("Failed to write to {} file", QBE_EXECUTABLE_PATH))?;

        let metadata = exe_file.metadata()?;
        let mut permission = metadata.permissions();
        permission.set_mode(0o777);
        std::fs::set_permissions(QBE_EXECUTABLE_PATH, permission.clone()).with_context(|| {
            format!(
                "Failed to elevate qbe binary file permission: {:?}",
                permission
            )
        })?;
        Ok(Self {})
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
