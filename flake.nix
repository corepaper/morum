{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rustOverlay.url = "github:oxalica/rust-overlay";
    rustOverlay.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, rustOverlay }: let
    pkgs = nixpkgs.legacyPackages."x86_64-linux";
    rustPkgs = pkgs.extend rustOverlay.overlay;
    rustStable = rustPkgs.rustChannelOf {
      channel = "1.65.0";
    };
    rustWasm = rustStable.default.override {
      targets = [ "wasm32-unknown-unknown" ];
    };
    rustWasmPlatform = pkgs.makeRustPlatform {
      rustc = rustWasm;
      cargo = rustWasm;
    };

    morumInputs = with pkgs; [
      wasm-bindgen-cli binaryen clang pkg-config
      trunk nodePackages.sass
    ];
  in {
    legacyPackages."x86_64-linux".morum = with pkgs; rustWasmPlatform.buildRustPackage rec {
      pname = "morum";
      version = "0.1.0";

      src = ./.;

      cargoSha256 = "sha256-/p8mVTwXBi5ThAtDg6RGT2O/N0ItTuiA2oVK0HrDBo8=";
      nativeBuildInputs = morumInputs;
    };

    devShell."x86_64-linux" = with pkgs; mkShell {
      buildInputs = morumInputs ++ [ rustWasm mold ];
    };
  };
}
