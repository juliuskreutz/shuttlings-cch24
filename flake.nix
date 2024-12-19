{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      nixpkgs,
      rust-overlay,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
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
          version = "19.0.0";

          src = pkgs.fetchCrate {
            inherit pname version;
            hash = "sha256-61pQ8NFOzXhz1MBx/SBlEk6OgtwA5QAU1G4rwqvHpws=";
          };

          cargoHash = "sha256-+2Jvrm5O8qtDNtnEwwXsDONnqPbAjhtC3DqaValoZ3o=";
        };
      in
      {
        devShells.default =
          with pkgs;
          mkShell {
            buildInputs = [
              rust-bin.stable.latest.default
              rust-analyzer
              taplo
              cargo-watch
              sqlx-cli
              pgformatter
              shuttle
              cch24-validator
            ];
          };
      }
    );
}
