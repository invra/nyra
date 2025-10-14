#   SPDX-License-Identifier: Unlicense
#   Project: Nyra
#   File: flake.nix
#   Authors: Invra
#   Notes: Nix flake output or something

{
  description = "Flake for Nyra";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    treefmt-nix.url = "github:numtide/treefmt-nix";
    csharp-ls.url = "github:invra/csharp-language-server";
    naersk.url = "github:nix-community/naersk";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    {
      nixpkgs,
      flake-utils,
      treefmt-nix,
      csharp-ls,
      rust-overlay,
      naersk,
      self,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ csharp-ls.overlays.default (import rust-overlay) ];
        };

        formatters =
          (treefmt-nix.lib.evalModule pkgs (_: {
            projectRootFile = ".git/config";
            programs = {
              nixfmt.enable = true;
              nixf-diagnose.enable = true;
              toml-sort.enable = true;
              rustfmt.enable = true;
            };
            settings.formatter = {
              rustfmt = {
                options = [
                  "--config"
                  "condense_wildcard_suffixes=true,tab_spaces=2,imports_layout=vertical"
                  "--style-edition"
                  "2024"
                ];
              };
            };
          })).config.build;
      in
      {
        devShells.default = pkgs.mkShell rec {
          meta.license = pkgs.lib.licenses.unlicense;

          buildInputs =
            with pkgs;
            [
              bacon
              rust-bin.nightly.latest.default
              clippy
              pkg-config
              rust-analyzer
            ]
            ++ nixpkgs.lib.optionals pkgs.stdenv.isLinux [
              xorg.libxcb
              xorg.xcbutil
              libxkbcommon
              libxkbcommon_8
            ]
            ++ nixpkgs.lib.optionals pkgs.stdenv.isDarwin [
              apple-sdk_15
            ];

          runtimeLibs = nixpkgs.lib.optionals pkgs.stdenv.isLinux (
            with pkgs;
            [
              expat
              fontconfig
              freetype
              freetype.dev
              libGL
              wayland
              xorg.libXi
              xorg.libX11
              xorg.xcbutil
              xorg.libXrandr
              xorg.libXcursor
              xorg.libxcb
              xorg.xcbutil
              libxkbcommon
            ]
          );

          LD_LIBRARY_PATH = builtins.foldl' (a: b: "${a}:${b}/lib") "${pkgs.vulkan-loader}/lib" runtimeLibs;

          shellHook =
            if !pkgs.stdenv.isDarwin then
              ''
                #!/bin/bash
                COMMAND=$(awk -F: -v user=$USER 'user == $1 {print $NF}' /etc/passwd)
                if [ "$COMMAND" != *bash* ]; then
                  $COMMAND
                  exit
                fi
              ''
            else
              ''
                #!/bin/bash
                COMMAND=$(dscl . -read $HOME 'UserShell' | grep --only-matching '/.*')
                if [ "$COMMAND" != *bash* ]; then
                  $COMMAND
                  exit
                fi
              '';
        };

        packages.default =
          with pkgs;
          rustPlatform.buildRustPackage rec {
            name = "nyra";

            src = ./.;
            cargoHash = "sha256-dd3V/doJJeQrwFRrO4pvJlXd8WZ+KQf0tXgWvCPiR+s=";

            nativeBuildInputs = nixpkgs.lib.optionals pkgs.stdenv.isLinux [
              pkg-config
              xorg.libxcb
              xorg.xcbutil
              libxkbcommon
              libxkbcommon_8
            ];

            buildInputs = [
              libiconv
            ];

            runtimeLibs = nixpkgs.lib.optionals stdenv.isLinux [
              expat
              fontconfig
              freetype
              freetype.dev
              libGL
              pkg-config
              xorg.libX11
              xorg.libXcursor
              xorg.libXi
              xorg.libXrandr
              wayland
              libxkbcommon
            ];

            LD_LIBRARY_PATH = builtins.foldl' (a: b: "${a}:${b}/lib") "${pkgs.vulkan-loader}/lib" runtimeLibs;
          };

        formatter = formatters.wrapper;
        checks.formatting = formatters.check self;
      }
    );
}
