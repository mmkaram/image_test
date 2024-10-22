{ pkgs ? import (fetchTarball "https://github.com/NixOS/nixpkgs/archive/9fd253a936a52106f92d064a3a44cfcecd997e64.tar.gz") {} }:
pkgs.mkShell {
  buildInputs = with pkgs; [
    gtk2
    rustup
    glib
    ffmpeg
    libva
    pkg-config
    (opencv.override {enableGtk2 = true;})
  ];
  # Set LIBCLANG_PATH to the location of libclang
  shellHook = ''
    export LIBCLANG_PATH="${pkgs.llvmPackages.libclang.lib}/lib";
  '';
}
