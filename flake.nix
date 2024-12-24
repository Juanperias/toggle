{
  description = "Toggle flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    crane.url = "github:ipetkov/crane";
    flake-utils.url = "github:numtide/flake-utils";
    fenix.url = "github:nix-community/fenix";
  };

  outputs = { self, nixpkgs, crane, flake-utils, fenix, ... }@inputs :
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        crane = inputs.crane.mkLib pkgs;
        toolchain = fenix.packages.${system}.fromToolchainFile {
          file = ./kernel/rust-toolchain.toml;
          sha256 = "sha256-jRLUIHKLOmiqsMgA/lauhY7FzfahJqkR0CUsqyfUMRQ=";
        };
        craneLib = crane.overrideToolchain toolchain;
      

        commonArgs = {
          src = craneLib.cleanCargoSource ./.;
          strictDeps = true;
          cargoToml = ./kernel/Cargo.toml;
          cargoVendorDir = ./kernel/Cargo.lock;

          buildInputs = [
          ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [];

        };

        my-crate = craneLib.buildPackage (commonArgs // {
          cargoArtifacts = craneLib.buildDepsOnly commonArgs;
        });
      in
      {
        devShells.default = craneLib.devShell {
          packages = [
            toolchain
            pkgs.rust-analyzer
            pkgs.gnumake
            pkgs.just 
            pkgs.xorriso
            pkgs.qemu
            pkgs.git  
            pkgs.gdb
          ];
        };
      });
}
