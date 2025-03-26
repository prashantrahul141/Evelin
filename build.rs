use std::{path::Path, process::Command};

const QBE_RELEASE_URL: &str = "https://c9x.me/compile/release/qbe-1.2.tar.xz";
const EXTERNAL_DIR: &str = "./src/external/";
const QBE_TAR_FILE: &str = "qbe.tar";

fn main() {
    println!("cargo::rerun-if-changed=src/external/qbe-1.2/main.c");

    // download qbe source.
    let status = Command::new("wget")
        .args(["-O", QBE_TAR_FILE, QBE_RELEASE_URL])
        .current_dir(EXTERNAL_DIR)
        .status()
        .expect("Failed to download qbe source. please make sure wget is installed.");

    if !status.success() {
        panic!("Failed to download qbe source.");
    }

    // untar it.
    let status = Command::new("tar")
        .args(["-xf", QBE_TAR_FILE])
        .current_dir(EXTERNAL_DIR)
        .status()
        .expect("Failed to untar qbe tar file.");

    if !status.success() {
        panic!("Failed to untar qbe source. please make sure gnu tar is installed.");
    }

    // compile it.
    let status = Command::new("make")
        .args(["-j", "8"])
        .current_dir(Path::join(Path::new(EXTERNAL_DIR), "qbe-1.2"))
        .status()
        .expect("Failed to compile qbe.");

    if !status.success() {
        panic!("Failed to build qbe from make.");
    }
}
