{
  description = "A nix flake for the spling project";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    crane.url = "github:ipetkov/crane";

  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, crane }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;


        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;
        src = craneLib.cleanCargoSource ./.;

        nativeBuildInputs = with pkgs; [
          pkg-config
          mold
          makeWrapper
        ];

        buildInputs = with pkgs; [
          alsa-lib
          udev
          mesa
          libglvnd
          vulkan-loader
          wayland
          libxkbcommon
          xorg.libX11
          xorg.libXcursor
          xorg.libXrandr
          xorg.libXi
        ];

        libPath = pkgs.lib.makeLibraryPath buildInputs;

        commonArgs = {
          inherit src nativeBuildInputs buildInputs;
        };

        cargoArtifacts = craneLib.buildDepsOnly commonArgs;

        spling-pkg = craneLib.buildPackage (commonArgs // {
          inherit cargoArtifacts;
          postInstall = ''
            wrapProgram $out/bin/spling --prefix LD_LIBRARY_PATH : "${libPath}"
          '';
        });

      in
      {
        packages.default = spling-pkg;

        apps.default = flake-utils.lib.mkApp {
          drv = spling-pkg;
        };

        devShells.default = craneLib.devShell {
          inputsFrom = [ spling-pkg ];
          LD_LIBRARY_PATH = libPath;
        };
      }
    );
}
