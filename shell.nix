let
    rust_overlay = import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz");
    nixpkgs = import <nixpkgs> { overlays = [ rust_overlay ]; };
    rust_toolchain = nixpkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
in

with nixpkgs;

pkgs.mkShell {
    buildInputs = with pkgs; [ stdenv ];
    # nativeBuildInputs is usually what you want -- tools you need to run
    nativeBuildInputs = with pkgs; [
        rust_toolchain
        cargo
        trunk
        wasm-bindgen-cli
        wasm-pack
        nodejs
    ];
}
