{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  inputs.systems.url = "github:nix-systems/default";
  inputs.flake-utils = {
    url = "github:numtide/flake-utils";
    inputs.systems.follows = "systems";
  };

  outputs =
    {
      nixpkgs,
      flake-utils,
      self,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        devShells.default = pkgs.mkShell {
          packages = [
            pkgs.rustup
            pkgs.bash
            pkgs.lua
            pkgs.python3
            pkgs.nodejs-slim
            pkgs.php
            pkgs.ruby
            # use a fake swift program while these issues are open
            # https://github.com/NixOS/nixpkgs/issues/462451
            # https://github.com/NixOS/nixpkgs/issues/311565
            # replace the following line with pkgs.swift when they are closed
            self.packages.${system}.fake_swift
          ];
          formatter = pkgs.nixfmt;
        };

        packages.fake_swift = pkgs.writeShellScriptBin "swift" ''
          echo "Hello, World!"
        '';
      }
    );
}
