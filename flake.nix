{
  description = "noway - An attempt to create fast and configurable Wayland compositor in Rust";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    devshell = {
      url = "github:mora-unie-youer/devshell";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
    };
  };

  outputs = inputs:
    inputs.flake-utils.lib.eachSystem [ "x86_64-linux" ] (system:
      let
        pkgs = import inputs.nixpkgs {
          inherit system;
          overlays = [
            inputs.devshell.overlays.default
            inputs.rust-overlay.overlays.default
          ];
        };
      in {
        devShells.default = import ./devshell.nix { inherit pkgs; };
      }
    );
}
