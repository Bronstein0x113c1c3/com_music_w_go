{ pkgs ? import <nixpkgs> { } }:

with pkgs;

mkShell {
  buildInputs = [
    go
    gcc
    gotools
    gopls
    delve
    pkg-config
    alsa-lib
  ];
}

