{
  description = "Dev environment for Tauri and Leptos.";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { nixpkgs, flake-utils, fenix, ... }@inputs:
  flake-utils.lib.eachDefaultSystem (system:
  let
    pkgs = import nixpkgs {
      inherit system;
      config = {
        allowUnfree = true;
      };
    };
  in
  {
    devShell = pkgs.mkShell {
      buildInputs = with pkgs; [
        webkitgtk_4_1
        gnumake
        gcc
        curl
        wget
        file
        librsvg
        pkg-config
        dbus
        openssl
        glib
        gtk4
        libsoup
        libappindicator
        trunk
        sass
        (with fenix.packages.${system}; combine [
          latest.rustc
          latest.cargo
          latest.rust-analyzer
          latest.cargo-leptos
          latest.cargo-generate
          targets.wasm32-unknown-unknown.latest.rust-std
        ])
      ];

      shellHook =
        ''
          echo "$(rustc --version)"
          echo "$(cargo tauri --version)"
          echo "Environment ready with Rust Nightly.";
        '';
      };
    });
  }
