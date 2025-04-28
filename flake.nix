{
  description = "The Evelin Programming Language";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?rev=eb0e0f21f15c559d2ac7633dc81d079d1caf5f5f";
  };

  outputs =
    { self, nixpkgs }:
    let
      supportedSystems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];
      forAllSystems =
        f: nixpkgs.lib.genAttrs supportedSystems (system: f nixpkgs.legacyPackages.${system});
    in
    {
      devShells = forAllSystems (pkgs: {
        default = pkgs.mkShell {
          packages = with pkgs; [
            cargo
            rustc
            rustfmt
            rust-analyzer
            rustPackages.clippy
            binutils
            git
          ];
        };
      });
    };
}
