## Installation Guide

Evelin is currently supported on amd64 (linux and MacOS), arm64, and riscv64. It uses [QBE backend](https://c9x.me/compile/) to generate platform-dependent machine code and the host machine's C compiler to build the final binary.

Evelin can be either built from source or you can download existing binaries from the [releases page](https://github.com/prashantrahul141/Evelin/releases).

### Building
You'll need the [rust toolchain](https://rustup.rs/).

Recursively clone the repo:
```sh
git clone --recurse-submodules https://github.com/prashantrahul141/Evelin
```

Build using `cargo`:

(This also downloads and compiles QBE using the host machine's C compiler)
```sh
cargo build
```

Run one of the [example](./examples/) files:
```sh
cargo run -- ./examples/01.eve
```

This will generate an `out` file, which you can run:
```sh
./out
Hello, world!
```

See the full help message using `--help`
```sh
cargo run -- --help

The Evelin Programming Language

Usage: evelin [OPTIONS] [FILE]...

Arguments:
  [FILE]...  Evelin source files path

Options:
  -c, --cc <CC>                 C compiler [default: cc]
  -d, --debug <DEBUG>           Turn debugging information on [default: off] [possible values: off, error, debug, trace]
  -o, --out <OUT>               Out file name [default: out]
  -l, --lib_name <LIB_NAME>...  External library name passed to the linker as -l<lib1> -l<lib2>
  -L, --lib_path <LIB_PATH>...  External library directory passed to the linker as -L<path_1> -L<path_2>
  -h, --help                    Print help
  -V, --version                 Print version
```
