{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs {
        inherit system;
        config.allowUnfree = true;
      };
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        buildInputs = with pkgs; [
          gtk2
          rustup
          glib
          ffmpeg
          libva
          pkg-config
          (opencv.override { enableGtk2 = true; })
        ];
        shellHook = ''
          export LIBCLANG_PATH="${pkgs.llvmPackages.libclang.lib}/lib";
        '';
      };

      # Define a default package
      packages.${system}.default = pkgs.stdenv.mkDerivation {
        name = "my-project";
        src = ./.;  # Use the current directory as the source
        buildInputs = with pkgs; [
          gtk2
          rustc
          cargo
          glib
          ffmpeg
          libva
          pkg-config
          (opencv.override { enableGtk2 = true; })
        ];
        buildPhase = ''
		cargo build
        '';
        installPhase = ''
          # Add your install commands here
          # For example:
          # mkdir -p $out/bin
          # cp target/release/my-binary $out/bin/
        '';
      };

      # Optionally, you can also define a default app
      apps.${system}.default = {
        type = "app";
        program = "${self.packages.${system}.default}/bin/image-test";
      };
    };
}
