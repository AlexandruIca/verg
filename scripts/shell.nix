let
    pkgs = import (fetchTarball("https://github.com/NixOS/nixpkgs/archive/573095944e7c1d58d30fc679c81af63668b54056.tar.gz")) {};
in pkgs.mkShell {
    name = "rust-environment";
    buildInputs = [
        pkgs.cargo
        pkgs.rustc
        pkgs.rustfmt
        pkgs.clippy
        pkgs.cargo-tarpaulin
        pkgs.ghc
        pkgs.texlive.combined.scheme-full
    ];
    RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}
