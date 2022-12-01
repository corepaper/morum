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
      openssl_1_1 openssl_1_1.dev trunk nodePackages.sass
    ];
  in {
    legacyPackages."x86_64-linux".morum = with pkgs; rustWasmPlatform.buildRustPackage rec {
      pname = "morum";
      version = "0.1.0";

      src = ./.;

      cargoSha256 = "sha256-jK2Z+4tyUsjxu2l14w8IfQM4z84bYKr0s/w8tQF43hA=";
      nativeBuildInputs = morumInputs;

      OPENSSL_LIB_DIR = "${openssl_1_1.out}/lib";
      OPENSSL_DIR = "${openssl_1_1.dev}";
    };

    devShell."x86_64-linux" = with pkgs; mkShell {
      buildInputs = morumInputs ++ [ rustWasm mold ];
    };
  };
}
