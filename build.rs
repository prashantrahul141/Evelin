use std::process::Command;

const QBE_ROOT_DIR: &str = "./external/qbe-1.2/";
const EXTERNAL_ROOT_DIR: &str = "./external/";
const QBE_URI: &str = "https://c9x.me/compile/release/qbe-1.2.tar.xz";

macro_rules! p {
    ($($tokens: tt)*) => {
        println!("cargo::warning={}", format!($($tokens)*))
    }
}

fn main() {
    println!("cargo::rerun-if-changed=external/qbe-1.2/main.c");
    p!("Rerun build.");

    if let Err(e) = Command::new("wget")
        .args(["-nc", QBE_URI])
        .current_dir(EXTERNAL_ROOT_DIR)
        .output()
    {
        panic!("Failed to download qbe tar file : {}", e);
    }

    if let Err(e) = Command::new("tar")
        .args(["vxf", "qbe-1.2.tar.xz"])
        .current_dir(EXTERNAL_ROOT_DIR)
        .output()
    {
        panic!("Failed to untar qbe file: {}", e);
    }

    if let Err(e) = Command::new("make")
        .args(["-j", "16"])
        .current_dir(QBE_ROOT_DIR)
        .output()
    {
        panic!("Failed while building qbe : {}", e);
    }
}
