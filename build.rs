use std::process::Command;

const QBE_ROOT_DIR: &str = "./src/external/qbe-1.2/";
const EXTERNAL_ROOT_DIR: &str = "./src/external/";
const QBE_URI: &str = "https://c9x.me/compile/release/qbe-1.2.tar.xz";

macro_rules! p {
    ($($tokens: tt)*) => {
        println!("cargo::warning={}", format!($($tokens)*))
    }
}

fn main() {
    println!("cargo::rerun-if-changed=src/external/qbe-1.2/main.c");
    p!("Rerun build.");

    let status = Command::new("wget")
        .args(&["-nc", QBE_URI])
        .current_dir(EXTERNAL_ROOT_DIR)
        .output();

    if !status.is_ok() {
        panic!("Failed to download qbe tar file.");
    }

    let status = Command::new("tar")
        .args(&["vxf", "qbe-1.2.tar.xz"])
        .current_dir(EXTERNAL_ROOT_DIR)
        .output();

    if !status.is_ok() {
        panic!("Failed to extract qbe tar file.");
    }

    let status = Command::new("make")
        .args(&["-j", "16"])
        .current_dir(QBE_ROOT_DIR)
        .output();

    if !status.is_ok() {
        panic!("Failed to build qbe using make.");
    }
}
