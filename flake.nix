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
    criome-cozo = { url = "github:LiGoldragon/criome-cozo"; flake = false; };
  };

  outputs = { self, nixpkgs, flake-utils, crane, fenix, criome-cozo, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        rustToolchain = fenix.packages.${system}.latest.toolchain;
        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

        # Include .cozo files alongside standard cargo sources
        cozoFilter = path: _type: builtins.match ".*\\.cozo$" path != null;
        sourceFilter = path: type:
          (cozoFilter path type) || (craneLib.filterCargoSources path type);
        src = pkgs.lib.cleanSourceWith {
          src = ./.;
          filter = sourceFilter;
        };

        commonArgs = {
          inherit src;
          pname = "samskara-lojix-contract";
          cargoExtraArgs = "--lib";
          # Place path deps where Cargo.toml expects them (../criome-cozo)
          postUnpack = ''
            depDir=$(dirname $sourceRoot)
            cp -rL ${criome-cozo} $depDir/criome-cozo
          '';
        };
        cargoArtifacts = craneLib.buildDepsOnly commonArgs;
      in
      {
        packages.default = craneLib.buildPackage (commonArgs // {
          inherit cargoArtifacts;
        });

        checks = {
          build = craneLib.buildPackage (commonArgs // {
            inherit cargoArtifacts;
          });
          tests = craneLib.cargoTest (commonArgs // {
            inherit cargoArtifacts;
          });
        };

        devShells.default = craneLib.devShell {
          packages = [ pkgs.rust-analyzer pkgs.jujutsu ];
        };
      }
    );
}
