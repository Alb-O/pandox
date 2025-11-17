{
  description = "proj";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
    dx7.url = "github:NixOS/nixpkgs/master";
  };

  outputs =
    {
      self,
      nixpkgs,
      dx7,
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
      in
      {
        devShells.default =
          with pkgs;
          mkShell {
            buildInputs = [
              openssl
              pkg-config
              pandoc
              tailwindcss_4
              (rust-bin.nightly.latest.default.override {
                targets = [ "wasm32-unknown-unknown" ];
                extensions = [ "rust-src" ];
              })
              dx7.legacyPackages.${pkgs.system}.dioxus-cli
            ];
            shellHook = ''
              cargo install wasm-bindgen-cli --all-features -q
              export PATH=$CARGO_HOME/bin:$PATH
            '';
          };
      }
    );
}
