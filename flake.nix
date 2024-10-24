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

          pyright
          (python3.withPackages (ps: with ps; [black sympy networkx]))
          (haskellPackages.ghcWithPackages (ps: with ps; [random matrix split MemoTrie parallel memoize]))

          kotlin-language-server
          kotlin
          jdk
        ];
        shellHook = ''
          export PATH="$PATH:${pkgs.writeScriptBin "runkotlin" ''
            #!/usr/bin/env bash
            set -e
            kotlinc $1 -include-runtime -d /tmp/run.jar
            java -jar /tmp/run.jar
          ''}/bin"
        '';
      };
    });
  };
}
