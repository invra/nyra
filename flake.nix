{
  description = "Flake for Dotnet";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    treefmt-nix.url = "github:numtide/treefmt-nix";
  };

  outputs =
    {
      nixpkgs,
      flake-utils,
      treefmt-nix,
      self,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
        formatters =
          (treefmt-nix.lib.evalModule pkgs (_: {
            projectRootFile = ".git/config";
            programs = {
              nixfmt.enable = true;
              nixf-diagnose.enable = true;
            };
            settings.formatter.dotnet-format = {
              command = "${pkgs.dotnetCorePackages.sdk_9_0-bin}/bin/dotnet";
              options = [
                "format"
              ];
              includes = [ "*.csproj" ];
            };
          })).config.build;
      in
      {
        devShells.default = pkgs.mkShell {
          meta = {
            license = pkgs.lib.licenses.unlicense;
          };
          buildInputs = with pkgs; [
            dotnetCorePackages.sdk_9_0-bin
            omnisharp-roslyn
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

          dotnet-sdk = pkgs.dotnetCorePackages.sdk_9_0-bin;
          dotnet-runtime = pkgs.dotnetCorePackages.runtime_9_0;
          nugetDeps = ./deps.json;

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
