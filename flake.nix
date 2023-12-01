{
  description = "simple rust flake";

  inputs = {nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";};

  outputs = {
    self,
    nixpkgs,
  }: let
    allSystems = [
      "x86_64-linux" # 64bit AMD/Intel x86
      "aarch64-linux" # 64bit ARM Linux
      "x86_64-darwin" # 64bit AMD/Intel macOS
      "aarch64-darwin" # 64bit ARM macOS
    ];

    forAllSystems = fn:
      nixpkgs.lib.genAttrs allSystems
      (system: fn {pkgs = import nixpkgs {inherit system;};});
  in {
    devShells = forAllSystems ({pkgs}: {
      default = pkgs.mkShell {
        name = "nix";
        packages = with pkgs; [
          rustc
          cargo
          rustfmt
          rustPackages.clippy
          rust-analyzer
        ];
      };
    });

    packages = forAllSystems ({pkgs}: {
      default = pkgs.rustPlatform.buildRustPackage {
        pname = "rust";
        version = "0.0.1";

        src = ./rust;

        cargoLock = {
          lockFile = ./rust/Cargo.lock;
        };

        # buildInputs = with pkgs; [];
        # nativeBuildInputs = with pkgs; [
        #   pkg-config
        # ];
      };
    });
  };
}
