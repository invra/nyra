{
  description = "Flake for Go-lang development";

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
            settings.formatter.nufmt = {
              command = "${pkgs.dotnetCorePackages.sdk_10_0-bin}/bin/dotnet";
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
            dotnetCorePackages.sdk_10_0-bin
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
          name = "terry-davis";
          src = ./.;
          # vendorHash = null;
        };

        formatter = formatters.wrapper;
        checks.formatting = formatters.check self;
      }
    );
}
