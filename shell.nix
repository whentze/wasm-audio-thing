{ pkgs ? import <nixpkgs> {} }:
  pkgs.mkShell {
    nativeBuildInputs = with pkgs; [
      rustup
    ];
    shellHook = ''
      export PATH=$PATH:~/.cargo/bin
      '';
  }
