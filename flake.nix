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
          version = "2.0.1";

          src = pkgs.fetchCrate {
            inherit pname version;
            hash = "sha256-AdKFVGvRe3xG3bzaaAJ95NSWjJ4oCgmB8Y7UcaDVOiM=";
          };

          cargoHash = "sha256-1BaR4Rc62INH9rP4y1D2O15cyT7H1D1amXK4hKC8xmk=";
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
