{ pkgs ? import <nixpkgs> { } }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    cargo
    cargo-outdated
    rustc
    rustfmt
    rust-analyzer
    clippy

    pkg-config
    hidapi
    libusb1
  ];

  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}
