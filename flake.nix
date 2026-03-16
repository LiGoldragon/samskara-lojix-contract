{
  description = "Datalog schema contract between Samskara and Lojix agents";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    crane.url = "github:ipetkov/crane";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    criome-cozo-src = { url = "github:LiGoldragon/criome-cozo"; flake = false; };
  };

  outputs = inputs@{ self, nixpkgs, flake-utils, crane, fenix, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        rustToolchain = fenix.packages.${system}.latest.toolchain;
        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;
        src = craneLib.cleanCargoSource ./.;
      in
      {
        packages.default = craneLib.buildPackage {
          inherit src;
          pname = "samskara-lojix-contract";
          cargoExtraArgs = "--lib";
        };

        checks.default = craneLib.buildPackage {
          inherit src;
          pname = "samskara-lojix-contract";
          cargoExtraArgs = "--lib";
        };

        devShells.default = craneLib.devShell {
          packages = [ pkgs.rust-analyzer ];
        };
      }
    );
}
