{ pkgs ? import <nixpkgs> {} }:
# needs pkg-config
pkgs.mkShell {
  buildInputs = with pkgs; [
    pkg-config
    rustup
    cargo
    ffmpeg
  ];
}