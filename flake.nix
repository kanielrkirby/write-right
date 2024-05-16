{
  inputs = {
    nixpkgs.url     = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix.url       = "github:nix-community/fenix";
  };

  outputs = { nixpkgs, fenix, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            openssl
            pkg-config
            cacert
            cargo-make
            cargo-leptos
            cargo-whatfeatures
            sass
            tailwindcss
            mold
            sccache
            evcxr
            bacon
            trunk
            (with fenix.packages.${system}; combine [
              latest.toolchain
              targets.wasm32-unknown-unknown.latest.toolchain
            ])
          ];

          shellHook = ''
            echo "$(
              echo
              echo "Installed Targets:"
              echo
              echo "$(${pkgs.rustup}/bin/rustup target list --installed | sed 's/^.*/  \0/')"
            )"
            echo "$(
              echo
              echo "Environment Packages:"
              echo
              echo "$(
                cargo --version | sed 's/ (.*$//'
                trunk --version
                evcxr --version
                bacon --version
                sccache --version
                cargo leptos --version
                cargo make --version
                cargo whatfeatures --version
                echo "pkg-config $(pkg-config --version)"
                mold --version | sed 's/ (.*$//'
                sass --version
                tailwindcss --help | head -n 2 | tail -n 1
                openssl version | sed 's/\([^ ]* [^ ]*\).*/\1/';
              )" | sed 's/^.*/  \0/'
            )"
            echo
          '';
        };
      }
    );
}
