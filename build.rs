use std::error::Error;
use std::ffi::CStr;
use std::io::Write;
use std::{env, fs, path::PathBuf};

// Wraps `uname` output fields
struct SysInfo {
    pub sysname: String,
    pub machine: String,
}

// basically a constructor for SysInfo
// Can panic
fn uname() -> Result<SysInfo, Box<dyn Error>> {
    let mut buf = unsafe { std::mem::zeroed() };
    let success = unsafe { libc::uname(&mut buf) };
    if success != 0 {}
    let sysname = unsafe { CStr::from_ptr(buf.sysname.as_ptr()) }
        .to_string_lossy()
        .into_owned();
    let machine = unsafe { CStr::from_ptr(buf.machine.as_ptr()) }
        .to_string_lossy()
        .into_owned();

    Ok(SysInfo { sysname, machine })
}

// returns qbe's config.h content depending on build machine
fn get_qbe_config(sysinfo: SysInfo) -> String {
    // apple
    if sysinfo.sysname.find("Darwin").is_some() {
        // apple-arm64
        if sysinfo.machine.find("arm64").is_some() {
            "#define Deftgt T_arm64_apple".into()
        }
        // apple-amd64
        else {
            "#define Deftgt T_amd64_apple".into()
        }
    }
    // all other devices
    else {
        // arm64
        if sysinfo.machine.find("aarch64").is_some() || sysinfo.machine.find("aarch64").is_some() {
            "#define Deftgt T_arm64".into()
        } else if sysinfo.machine.find("riscv64").is_some() {
            "#define Deftgt T_rv64".into()
        } else {
            "#define Deftgt T_amd64_sysv".into()
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
    write!(qbeconfigh, r#"{}"#, get_qbe_config(uname()?))?;

    c.compile("qbe");
    println!("cargo:rustc-link-search=vendor/qbe/");
    println!("cargo:rustc-link-lib=qbe");

    Ok(())
}
