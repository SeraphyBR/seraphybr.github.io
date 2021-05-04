# Use nixpkgs with oxalica rust-bin overlay
let
  rust_overlay = import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz");
  nixpkgs = import <nixpkgs> { overlays = [ rust_overlay ]; };
  rust_channel = nixpkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain;
in
# Avoid typing `nixpkgs.` before each package name
with nixpkgs;

pkgs.mkShell {
  buildInputs = with pkgs; [ stdenv zlib pkg-config openssl ];
  # nativeBuildInputs is usually what you want -- tools you need to run
  nativeBuildInputs = with pkgs; [
      # Rust Compiler and Cargo
      rust_channel
      # Cargo extensions
      cargo-edit
      cargo-make
      cargo-watch
      # WASM
      wasm-pack
      wasm-bindgen-cli
      # Rust WASM App Bundler
      trunk

      # Use steam-run, for a fhs enviroment, allow vscode code-lldb debugger to run
      # $ steam-run code .
      (steam.override {
        extraPkgs = pkgs: [ pkg-config zsh ];
        extraLibraries = pkgs: [ zlib openssl ];
        nativeOnly = true;
      }).run
  ];
}
