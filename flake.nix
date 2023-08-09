{
  description = "August";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    nixpkgs,
    flake-utils,
    ...
  }: let
    inherit (nixpkgs.lib) optional;
    systems = flake-utils.lib.system;
  in
    flake-utils.lib.eachDefaultSystem
    (system: let
      pkgs = import nixpkgs {
        inherit system;
      };
    in {
      devShell = pkgs.mkShell {
        nativeBuildInputs = with pkgs;
          [
            cargo-make
            nodejs
            nodePackages.prettier
            rustup
          ]
          ++ optional (system == systems.aarch64-darwin) [
            darwin.apple_sdk.frameworks.Security
            darwin.apple_sdk.frameworks.CoreFoundation
            darwin.apple_sdk.frameworks.CoreServices
          ];

        shellHook = ''
          export CARGO_INSTALL_ROOT="$(git rev-parse --show-toplevel)/.cargo"
          export PATH="$CARGO_INSTALL_ROOT/bin:$PATH"
        '';
      };
    });
}
