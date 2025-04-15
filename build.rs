use std::process::Command;

const QBE_ROOT_DIR: &str = "./external/qbe/";

macro_rules! p {
    ($($tokens: tt)*) => {
        println!("cargo::warning={}", format!($($tokens)*))
    }
}

fn main() {
    p!("Rerun build.");
    if let Err(e) = Command::new("make")
        .args(["-j", "16"])
        .current_dir(QBE_ROOT_DIR)
        .output()
    {
        panic!("Failed while building qbe : {}", e);
    }

    println!("cargo:rustc-link-search=external/qbe/");
    println!("cargo:rustc-link-lib=qbe");
}
