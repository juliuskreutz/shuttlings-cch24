{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    nixpkgs,
    rust-overlay,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        shuttle = pkgs.rustPlatform.buildRustPackage rec {
          pname = "cargo-shuttle";
          version = "0.49.0";

          doCheck = false;

          src = pkgs.fetchCrate {
            inherit pname version;
            hash = "sha256-yRIRu0F4BiM/KfKTwty+uzH4uvhbiYd+XxzAQoxyb6o=";
          };

          cargoHash = "sha256-8F8iUprhSFB3LZBEl5XNObvhc301/akGpP/nI9YFZ+4=";
        };

        cch24-validator = pkgs.rustPlatform.buildRustPackage rec {
          pname = "cch24-validator";
          version = "5.0.0";

          src = pkgs.fetchCrate {
            inherit pname version;
            hash = "sha256-Gk+DN80nnFrbwLFDw+VHir2YjSOr6KaINCYezjymfTs=";
          };

          cargoHash = "sha256-gLeK2flONRK2iHnIfbGRKcqqEpWkkQUxYwBHv+85hS0=";
        };
      in {
        devShells.default = with pkgs;
          mkShell {
            buildInputs = [
              rust-bin.stable.latest.default
              rust-analyzer
              taplo
              cargo-watch
              shuttle
              cch24-validator
            ];
          };
      }
    );
}
