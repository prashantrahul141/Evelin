use std::{path::Path, sync::Arc};

use std::process::{Command, Output};

use anyhow::Context;
use log::debug;

pub struct CCFlags {
    c_compiler: String,
    opt_level: i8,
    debugging: bool,
    outfile: String,
    lib_path: Vec<String>,
    lib_name: Vec<String>,
}

impl Default for CCFlags {
    fn default() -> Self {
        Self {
            c_compiler: "cc".into(),
            opt_level: 0,
            debugging: false,
            outfile: "out".into(),
            lib_path: vec![],
            lib_name: vec![],
        }
    }
}

/// Build object files into a single executable.
pub struct Build {
    pub files: Vec<Arc<Path>>,
    pub compiler_flags: CCFlags,
}

impl Default for Build {
    fn default() -> Self {
        Self::new()
    }
}

impl Build {
    pub fn new() -> Self {
        Self {
            files: vec![],
            compiler_flags: CCFlags::default(),
        }
    }
    ///  Set C compiler
    pub fn set_c_compiler<P: Into<String>>(&mut self, c: P) -> &mut Self {
        self.compiler_flags.c_compiler = c.into();
        self
    }

    /// Set optimization level
    pub fn set_opt<L: Into<i8>>(&mut self, l: L) -> &mut Self {
        self.compiler_flags.opt_level = l.into();
        self
    }

    /// Add a file which will be compiled
    pub fn file<P: AsRef<Path>>(&mut self, p: P) -> &mut Self {
        self.files.push(p.as_ref().into());
        self
    }

    #[allow(dead_code)]
    /// Add files which will be compiled
    pub fn files<P>(&mut self, p: P) -> &mut Self
    where
        P: IntoIterator,
        P::Item: AsRef<Path>,
    {
        for file in p.into_iter() {
            self.file(file);
        }
        self
    }

    /// output file name
    pub fn set_outfile<P: Into<String>>(&mut self, o: P) -> &mut Self {
        self.compiler_flags.outfile = o.into();
        self
    }

    /// set lib names
    pub fn set_lib_names<P>(&mut self, lib_names: P) -> &mut Self
    where
        P: IntoIterator,
        P::Item: Into<String>,
    {
        for lib_path in lib_names.into_iter() {
            self.compiler_flags.lib_name.push(lib_path.into());
        }
        self
    }

    /// set lib paths
    pub fn set_lib_paths<P>(&mut self, lib_paths: P) -> &mut Self
    where
        P: IntoIterator,
        P::Item: Into<String>,
    {
        for lib_path in lib_paths.into_iter() {
            self.compiler_flags.lib_path.push(lib_path.into());
        }
        self
    }

    pub fn compile(&mut self) -> anyhow::Result<Output> {
        let mut cmd = Command::new(&self.compiler_flags.c_compiler);

        let files = self
            .files
            .iter()
            .map(|x| x.to_str().unwrap())
            .collect::<Vec<_>>();
        cmd.args(files);

        cmd.arg(format!("-o{}", self.compiler_flags.outfile));

        if self.compiler_flags.debugging {
            cmd.arg("-g");
        } else {
            cmd.arg(format!("-O{}", self.compiler_flags.opt_level));
        }

        cmd.args(
            self.compiler_flags
                .lib_path
                .iter()
                .map(|x| format!("-L{}", x))
                .collect::<Vec<_>>(),
        );

        cmd.args(
            self.compiler_flags
                .lib_name
                .iter()
                .map(|x| format!("-l{}", x))
                .collect::<Vec<_>>(),
        );

        debug!("call c compiler {:?}", &cmd);

        cmd.output()
            .with_context(|| format!("Fail to compile with cmd = {:?}", cmd.get_args()))
    }
}
