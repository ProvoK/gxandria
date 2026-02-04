{
  description = "generic project flake";
  nixConfig.bash-prompt-prefix = "";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-25.11";
    nixpkgs-unstable.url = "github:nixos/nixpkgs/nixos-unstable";
    #nixpkgs-dx.url = "github:nixos/nixpkgs?ref=pull/407060/head";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    { ... }@inputs:
    inputs.flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = inputs.nixpkgs.legacyPackages.${system};
        unstablePkgs = import inputs.nixpkgs-unstable { inherit system; };
        #dxPkgs = import inputs.nixpkgs-dx { inherit system; };
        formatter = pkgs.nixfmt-rfc-style;
        wasm-bindgen-cli = pkgs.rustPlatform.buildRustPackage rec {
          pname = "wasm-bindgen-cli";
          version = "0.2.106";
          src = pkgs.fetchCrate {
            inherit pname version;
            sha256 = "sha256-M6WuGl7EruNopHZbqBpucu4RWz44/MSdv6f0zkYw+44=";
          };
          cargoHash = "sha256-ElDatyOwdKwHg3bNH/1pcxKI7LXkhsotlDPQjiLHBwA=";
          nativeBuildInputs = [ pkgs.pkg-config ];
          buildInputs = [ pkgs.openssl ];
          doCheck = false;
        };
      in
      {
        formatter = formatter;
        
        devShells.default = pkgs.mkShell {
          packages = [
            pkgs.gcc
            pkgs.pkg-config
            pkgs.openssl
            pkgs.dioxus-cli
            pkgs.cargo-make
            pkgs.tailwindcss_4
            pkgs.lazysql
            wasm-bindgen-cli

            # GUI dependencies
            pkgs.glib
            pkgs.gtk3
            pkgs.libsoup_3
            pkgs.webkitgtk_4_1
            pkgs.cairo
            pkgs.pango
            pkgs.atk
            pkgs.gdk-pixbuf
            pkgs.xdotool # often useful for automation/shortcuts
          ];
          shellHook = ''
            # libgcc.6 shit
            export LD_LIBRARY_PATH="${pkgs.stdenv.cc.cc.lib}/lib"; 

            # Dioxus 0.7.x+ telemetry disable
            export TELEMETRY="false";
            exec zsh
          '';
        };
      }
    );
}
