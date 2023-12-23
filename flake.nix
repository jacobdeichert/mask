{
  description = "A CLI task runner defined by a simple markdown file";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils.url = "github:numtide/flake-utils";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };

  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, crane }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            rust-overlay.overlays.default
          ];
        };

        inherit (pkgs) lib;
        
        rust-toolchain = (pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml);
        craneLib = (crane.mkLib pkgs).overrideToolchain rust-toolchain;
        binaryPackageMetadata = craneLib.crateNameFromCargoToml {
          cargoToml = ./mask/Cargo.toml;
        };

        buildInputs = [
            
        ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin (with pkgs.darwin.apple_sdk.frameworks; [
          pkgs.libiconv
          CoreFoundation
          Security
          SystemConfiguration
        ]);
        src = craneLib.cleanCargoSource ./.;
        
        crate = craneLib.buildPackage {
          inherit src;
          pname = binaryPackageMetadata.pname;
          version = binaryPackageMetadata.version;
          nativeBuildInputs = buildInputs;
          strictDeps = true;
          doCheck = false;
        };
      in
      {
        checks = {
          inherit crate;
        };

        packages.default = crate;

        apps.default = flake-utils.lib.mkApp {
          drv = crate;
        };

        devShells.default = pkgs.mkShell {
          buildInputs = [ rust-toolchain buildInputs pkgs.cargo-watch pkgs.rnix-lsp ];
        };
    });
}
