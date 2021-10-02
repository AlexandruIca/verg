let
    pkgs = import (fetchTarball("https://github.com/NixOS/nixpkgs/archive/ee90403e147b181300dffca5b0afa405e14f1945.tar.gz")) {};
in pkgs.mkShell {
    name = "rust-environment";
    buildInputs = [
        pkgs.cargo
        pkgs.rustc
        pkgs.rustfmt
        pkgs.clippy
        pkgs.cargo-tarpaulin
    ];
    shellHook = ''source ~/.bashrc'';
    RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}
