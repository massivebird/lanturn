{
  description = "Checks if your machine is freakin online bruv";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";
  };

  outputs = { self, nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };
      in
        with pkgs;
      {

        # `nix develop`
        devShell = mkShell {
          buildInputs = [
            cargo
            openssl
            pkg-config
            rustc
          ];
        };
      }
    );
}
