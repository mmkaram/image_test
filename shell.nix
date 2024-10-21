{ pkgs ? import <nixpkgs> {} }:
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
