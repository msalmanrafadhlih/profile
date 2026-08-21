{
  description = "Rust Development Flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";

    devenv.url = "github:cachix/devenv";
    crane.url = "github:ipetkov/crane";

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      nixpkgs,
      fenix,
      crane,
      flake-utils,
      ...
    }@inputs:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };

        # Toolchain juga didefinisikan di sini untuk naersk
        toolchain = fenix.packages.${system}.stable.toolchain;
        craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;

        commonArgs = {
          src = craneLib.cleanCargoSource (craneLib.path ./.);
          strictDeps = true;

          nativeBuildInputs = [ pkgs.pkg-config ];
          buildInputs = [ pkgs.openssl ];
        };

        cargoArtifacts = craneLib.buildDepsOnly commonArgs;

        my-app = craneLib.buildPackage (
          commonArgs
          // {
            inherit cargoArtifacts;
          }
        );
      in
      {
        devShells.default = inputs.devenv.lib.mkShell {
          inherit inputs pkgs;
          modules = [
            (import ./devenv.nix { templateInputs = inputs; })
            { setupRust.enable = true; }
          ];
        };

        # Build project sebagai paket Nix
        packages.default = my-app;
        checks = {
          clippy = craneLib.cargoClippy {
            src = craneLib.cleanCargoSource ./.;
            inherit cargoArtifacts;
            cargoClippyExtraArgs = "--all-targets -- --deny warnings";
          };
          fmt = craneLib.cargoFmt {
            src = craneLib.cleanCargoSource ./.;
          };
          nextest = craneLib.cargoNextest {
            src = craneLib.cleanCargoSource ./.;
            inherit cargoArtifacts;
          };
        };
      }
    )
    // {
      devenvModules.default = import ./devenv.nix { templateInputs = inputs; };
    };

  nixConfig = {
    extra-substituters = [
      "https://devenv.cachix.org"
      "https://nix-community.cachix.org"
    ];
    extra-trusted-public-keys = [
      "devenv.cachix.org-1:w1cLUi8dv3hnoSPGAuibQv+f9TZLr6cv/Hm9XgU50cw="
      "nix-community.cachix.org-1:mB9FSh9qf2dCimDSUo8Zy7bkq5CX+/rkCWyvRCYg3Fs="
    ];
  };
}
