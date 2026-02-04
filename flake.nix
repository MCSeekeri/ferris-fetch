{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      rust-overlay,
      treefmt-nix,
    }:
    flake-utils.lib.eachSystem [ "x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin" ] (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override { extensions = [ "rust-src" ]; };

        nativeBuildInputs = with pkgs; [
          rustToolchain
          pkg-config
        ];

        packageMeta = with pkgs.lib; {
          description = "A fast and cute system information tool written in Rust, featuring Ferris the crab!";
          homepage = "https://github.com/NORMAL-EX/ferris-fetch";
          license = licenses.mit;
          maintainers = [ ];
          mainProgram = "ferris-fetch";
        };

      in
      {
        formatter = (treefmt-nix.lib.evalModule pkgs ./treefmt.nix).config.build.wrapper;

        packages =
          let
            windowsPkgs = pkgs.pkgsCross.mingwW64;
          in
          {
            default = pkgs.rustPlatform.buildRustPackage {
              pname = "ferris-fetch";
              version = "0.1.0";

              src = ./.;

              cargoLock = {
                lockFile = ./Cargo.lock;
              };

              inherit nativeBuildInputs;

              meta = packageMeta;
            };

            ferris-fetch-windows = windowsPkgs.rustPlatform.buildRustPackage {
              pname = "ferris-fetch";
              version = "0.1.0";

              src = ./.;

              cargoLock = {
                lockFile = ./Cargo.lock;
              };

              nativeBuildInputs = with pkgs; [ pkg-config ];
              buildInputs = [ ];
              doCheck = false;

              meta = packageMeta // {
                platforms = pkgs.lib.platforms.windows;
                mainProgram = "ferris-fetch.exe";
              };
            };
          };

        devShells.default = pkgs.mkShell {
          nativeBuildInputs =
            nativeBuildInputs
            ++ (with pkgs; [
              cargo-watch
              rust-analyzer
              (treefmt-nix.lib.mkWrapper pkgs ./treefmt.nix)
            ]);

          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
        };

        apps.default = {
          type = "app";
          program = "${self.packages.${system}.default}/bin/ferris-fetch";
        };
      }
    );
}
