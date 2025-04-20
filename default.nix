{
  lib,
  rustPlatform,
  pkgs,
}:

rustPlatform.buildRustPackage rec {
  pname = "evelin";
  version = "0.0.1";

  src = ./.;

  cargoLock = {
    lockFile = ./Cargo.lock;
    allowBuiltinFetchGit = true;
  };
  meta = {
    description = "bro";
    homepage = "https://github.com/BurntSushi/ripgrep";
    license = lib.licenses.unlicense;
    maintainers = [ ];
  };

  nativeBuildInputs = with pkgs; [
    rustPlatform.cargoSetupHook
    setuptools-rust
    cargo
    rustc
    rustfmt
    rust-analyzer
    rustPackages.clippy
    binutils
    gnumake
    wget
    gnutar
  ];
}
