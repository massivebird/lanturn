{
  description = "Checks if your machine is freakin online bruv";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";
    naersk = {
      url = "github:nix-community/naersk/master";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, naersk, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        naersk-lib = pkgs.callPackage naersk { };
      in
        with pkgs;
      {
        packages.default = naersk-lib.buildPackage ./.;

        # for `nix develop`:
        shells.default = with pkgs;
          mkShell {
            buildInputs = [
              cargo
              openssl
              pkg-config
              clippy
              rust-analyzer
              rustc
            ];
          };
      }
    );
}
