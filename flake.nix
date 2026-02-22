{
  description = "generic project flake";
  nixConfig.bash-prompt-prefix = "";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-25.11";
    nixpkgs-unstable.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    { ... }@inputs:
    inputs.flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = inputs.nixpkgs.legacyPackages.${system};
        formatter = pkgs.nixfmt-rfc-style;
      in
      {
        formatter = formatter;

        devShells.default = pkgs.mkShell {
          packages = [
            pkgs.gcc
            pkgs.pkg-config
            pkgs.openssl
            pkgs.cargo-make
            pkgs.lazysql
            pkgs.sqlx-cli
            pkgs.cargo-make
            pkgs.cargo-dist
          ];
          shellHook = ''
            # libgcc.6 shit
            export LD_LIBRARY_PATH="${pkgs.stdenv.cc.cc.lib}/lib"; 
            exec zsh
          '';
        };
      }
    );
}
