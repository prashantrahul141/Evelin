use std::{
    ffi::{CStr, CString},
    ptr,
};

use anyhow::{Context, bail};

use super::Backend;

#[link(name = "qbe")]
unsafe extern "C" {
    fn qbe(input_fp: *mut libc::FILE, output_fp: *mut libc::FILE) -> libc::c_void;
}

/// Public qbe backend struct.
pub struct QbeBackend;

impl Default for QbeBackend {
    fn default() -> Self {
        Self {}
    }
}

impl Backend for QbeBackend {
    fn generate(&self, ir: String) -> Result<String, anyhow::Error> {
        let input = CString::new(ir).context("Failed to create C string from IR")?;
        let size = input.as_bytes().len();
        unsafe {
            let input_fp: *mut libc::FILE = libc::fmemopen(
                input.as_ptr() as *mut libc::c_void,
                size,
                CString::new("r")?.as_ptr(),
            );

            if input_fp.is_null() {
                bail!("Failed to create in-memory FILE*");
            }

            let mut output_ptr: *mut libc::c_char = ptr::null_mut();
            let mut output_size: libc::size_t = 0;

            let output_fp: *mut libc::FILE =
                libc::open_memstream(&mut output_ptr, &mut output_size);
            if output_fp.is_null() {
                bail!("Failed to create output FILE*");
            }

            // call qbe with input_fp, output_fp
            qbe(input_fp, output_fp);

            libc::fflush(output_fp);
            libc::fclose(input_fp);
            libc::fclose(output_fp);

            let c_str = CStr::from_ptr(output_ptr);
            let rs_out = c_str.to_string_lossy().into_owned();

            libc::free(output_ptr as *mut libc::c_void);

            Ok(rs_out)
        }
    }
}
