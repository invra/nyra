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
  };

  outputs =
    {
      nixpkgs,
      flake-utils,
      treefmt-nix,
      csharp-ls,
      self,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ csharp-ls.overlays.default ];
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
                  "condense_wildcard_suffixes=true,tab_spaces=2"
                  "--style-edition"
                  "2024"
                ];
              };
              dotnet-format = {
                command = "${pkgs.dotnetCorePackages.sdk_10_0-bin}/bin/dotnet";
                options = [
                  "format"
                ];
                includes = [ "*.csproj" ];
              };
            };
          })).config.build;
      in
      {
        devShells.default = pkgs.mkShell {
          meta.license = pkgs.lib.licenses.unlicense;
          buildInputs = with pkgs; [
            dotnetCorePackages.sdk_10_0-bin
            csharp-language-server
            rust-analyzer
            clippy
            cargo
            rustc
          ];

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

        packages.default = pkgs.buildDotnetModule {
          name = "nyra";

          src = ./.;
          nugetDeps = ./deps.jsonc;

          dotnet-sdk = pkgs.dotnetCorePackages.sdk_10_0-bin;
          dotnet-runtime = pkgs.dotnetCorePackages.runtime_10_0;

          nativeBuildInputs = with pkgs; [
            rustc
            cargo
            clang
            pkg-config
          ];

          buildInputs = with pkgs; [
            libiconv
          ];

          installPhase = ''
            dotnet publish -o $out/bin
            chmod +x $out/bin/nyra
          '';
        };

        formatter = formatters.wrapper;
        checks.formatting = formatters.check self;
      }
    );
}
