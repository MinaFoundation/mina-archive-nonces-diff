{
  description = "A rust development shell";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
    };
  };

  outputs = { nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

      in
      with pkgs;
      {
        devShells.default = mkShell {
          buildInputs = [
            rust-bin.stable.latest.default
          ];

          shellHook = ''
            cat <<EOF
            Welcome to the ❄️ rust development shell.

            EOF
            user_shell=$(getent passwd "$(whoami)" |cut -d: -f 7)
            exec "$user_shell"
          '';
        };
      }
    );
}

