use std::error::Error;
use std::io::Write;
use std::{env, fs, path::PathBuf};

// returns qbe's config.h content depending on build machine
fn get_qbe_config() -> String {
    #[cfg(not(target_os = "windows"))]
    {
        #[cfg(target_vendor = "apple")]
        {
            #[cfg(target_arch = "aarch64")]
            {
                // apple-arm64
                "#define Deftgt T_arm64_apple".into()
            }
            #[cfg(target_arch = "x86_64")]
            {
                // apple-amd64
                "#define Deftgt T_amd64_apple".into()
            }
        }

        #[cfg(not(target_vendor = "apple"))]
        {
            #[cfg(target_arch = "aarch64")]
            {
                "#define Deftgt T_arm64".into()
            }
            #[cfg(target_arch = "riscv64")]
            {
                "#define Deftgt T_rv64".into()
            }
            #[cfg(target_arch = "x86_64")]
            {
                "#define Deftgt T_amd64_sysv".into()
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut c = cc::Build::new();
    c.pic(true).opt_level(3).warnings(false);

    let root_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let vendor_dir = root_dir.join("vendor");
    let qbe_dir = vendor_dir.join("qbe");
    let files = &[
        "lib.c",
        "util.c",
        "parse.c",
        "abi.c",
        "cfg.c",
        "mem.c",
        "ssa.c",
        "alias.c",
        "load.c",
        "copy.c",
        "fold.c",
        "gvn.c",
        "gcm.c",
        "simpl.c",
        "live.c",
        "spill.c",
        "rega.c",
        "emit.c",
        "amd64/targ.c",
        "amd64/sysv.c",
        "amd64/isel.c",
        "amd64/emit.c",
        "arm64/targ.c",
        "arm64/abi.c",
        "arm64/isel.c",
        "arm64/emit.c",
        "rv64/targ.c",
        "rv64/abi.c",
        "rv64/isel.c",
        "rv64/emit.c",
    ];

    fs::create_dir_all(&qbe_dir)?; // to make sure qbe dir exists.
    for file in files {
        c.file(qbe_dir.join(file));
    }

    let mut qbeconfigh = fs::File::create(qbe_dir.join("config.h"))?;
    write!(qbeconfigh, r#"{}"#, get_qbe_config())?;

    c.compile("qbe");
    println!("cargo:rustc-link-search=vendor/qbe/");
    println!("cargo:rustc-link-lib=qbe");

    Ok(())
}
