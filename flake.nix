{
  description = "Al Azif core engine";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

    fenix.url = "github:nix-community/fenix";

    treefmt.url = "github:numtide/treefmt-nix";
    systems.url = "github:nix-systems/default";
  };

  nixConfig = {
    extra-substituters = ["https://nix-community.cachix.org"];
    extra-trusted-public-keys = ["nix-community.cachix.org-1:mB9FSh9qf2dCimDSUo8Zy7bkq5CX+/rkCWyvRCYg3Fs="];
  };

  outputs = {
    nixpkgs,
    fenix,
    treefmt,
    systems,
    ...
  }: let
    eachSystem = f:
      nixpkgs.lib.genAttrs
      (import systems) (system: f nixpkgs.legacyPackages.${system});

    rustToolchain = eachSystem (pkgs: fenix.packages.${pkgs.system}.default);
  in {
    formatter = eachSystem (pkgs:
      treefmt.lib.mkWrapper pkgs {
        projectRootFile = "flake.nix";

        programs = {
          alejandra.enable = true;
          rustfmt = {
            enable = true;
            package = rustToolchain.${pkgs.system}.rustfmt;
          };
          taplo.enable = true;
        };
      });

    devShells = eachSystem (pkgs: {
      default = pkgs.mkShell {
        buildInputs =
          (with pkgs; [
            alejandra
            cargo-expand
            taplo

            # For rug (gmp_mpfr_sys) crate
            diffutils
            gcc
            m4
          ])
          ++ [
            rustToolchain.${pkgs.system}.toolchain
          ];
      };
    });
  };
}
